#!/usr/bin/env python3
"""Extract, rather than retype, upstream resources, commands and converter data."""
from pathlib import Path
import re
import json
import subprocess
import sys
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
UP = ROOT / 'upstream/calculator/src'

def resw(name):
    return {d.attrib['name']: d.findtext('value') or '' for d in ET.parse(
        UP / 'Calculator/Resources/en-US' / name).getroot().findall('data')}

def wide(s):
    return 'L' + json.dumps(s, ensure_ascii=False)

resources = resw('CEngineStrings.resw')
resources.update(sDecimal='.', sThousand=',', sGrouping='3;0')
lines = ['// Generated from original Microsoft resources. See scripts/generate_engine_resources.py.',
         'static const std::unordered_map<std::wstring, std::wstring> engine_resources = {']
lines += [f'    {{{wide(k)}, {wide(v)}}},' for k,v in resources.items()]
lines += ['};']

# The original managed loader supplies data to the original native UnitConverter.
# Keep its identifiers, ordering, ratios, offsets and suggested (whimsical) units.
source = (UP / 'Calculator.ViewModels/DataLoaders/UnitConverterDataLoader.cs').read_text(encoding="utf-8")
ids = {k: int(v) for k,v in re.findall(r'private const int (\w+) = (\d+);', source)}
labels = resw('Resources.resw')
flags = dict(useSI=True, useUSCustomary=False, useFahrenheit=False,
             useWattInsteadOfKilowatt=False, usePyeong=False,
             useUSCustomaryAndFahrenheit=False, true=True, false=False)
def flag(s):
    s=s.strip()
    if ' && ' in s:
        return all(flag(v) for v in s.split(' && '))
    if s.startswith('_regionCode '):
        return ('!=' in s) # fixed SI defaults, region PT
    return not flags[s[1:]] if s.startswith('!') else flags[s]
cats=['Volume','Length','Weight','Temperature','Energy','Area','Speed','Time','Power','Data','Pressure','Angle']
lines += ['static void populate_units(std::vector<UnitConversionManager::Category>& categories,',
          '    std::unordered_map<int, std::vector<UnitConversionManager::Unit>>& units,',
          '    std::unordered_map<int, double>& factors,',
          '    std::unordered_map<int, std::unordered_map<int, UnitConversionManager::ConversionData>>& explicit_ratios) {']
for i,cat in enumerate(cats, 1):
    lines += [f'    categories.emplace_back({i}, {wide("Weight and mass" if cat == "Weight" else cat)}, {str(cat in ["Temperature","Power","Angle"]).lower()});']
unit_rows=re.findall(r'AddUnit\(ViewMode\.(\w+), (\w+), "([^"]+)", "([^"]+)", (\d+)([^;]*?)\);',source)
for cat,key,name,abbr,order,tail in sorted(unit_rows,key=lambda r:(cats.index(r[0]),int(r[4]))):
    options=[x.strip() for x in tail.split(',') if x.strip()]
    options += ['false']*(3-len(options))
    if key == 'Area_Pyeong': continue # this snapshot selects the SI, non-East-Asian defaults
    lines += [f'    units[{cats.index(cat)+1}].emplace_back({ids[key]}, {wide(labels[name.strip()])}, {wide(labels[abbr.strip()])}, '+', '.join(str(flag(v)).lower() for v in options)+');']
factor_section=source[source.index('// Build conversion factors'):source.index('// Build explicit conversion')]
for key,value in re.findall(r'\{\s*(\w+),\s*([\d.eE+\-]+)\s*\}',factor_section):
    lines += [f'    factors[{ids[key]}] = {value};']
explicit_section=source[source.index('var explicitConversions'):source.index('// Build unit lookup')]
for key,block in re.findall(r'\[(Temperature_\w+)\].*?\{(.*?)\n                \}',explicit_section,re.S):
    for target,ratio,offset,first in re.findall(r'\{ (\w+), \(([^,]+), ([^,]+), (true|false)\) \}',block):
        lines += [f'    explicit_ratios[{ids[key]}].emplace({ids[target]}, UnitConversionManager::ConversionData({ratio}, {offset}, {first}));']
# The original open-source currency feed is explicitly fictional. Its target/source
# rate convention is the reciprocal of the physical-unit factor convention.
currency_source=(UP/'Calculator.ViewModels/DataLoaders/CurrencyHttpClient.cs').read_text(encoding="utf-8")
def mock_array(field):
    return json.loads(re.search(r'private const string '+field+r' = @"(.*?)";',currency_source,re.S)[1].replace('""','"'))
currencies=mock_array('MockCurrencyStaticData')
rates={r['An']:r['Rt'] for r in mock_array('MockCurrencyConverterData')}
lines += ['    categories.emplace_back(0, L"Currency (upstream mock rates)", false);']
for i,currency in enumerate(currencies,1001):
    lines += [f'    units[0].emplace_back({i}, {wide(currency["CountryName"]+" — "+currency["CurrencyName"])}, {wide(currency["CurrencyCode"])}, {str(i==1001).lower()}, {str(i==1002).lower()}, false);',
              f'    factors[{i}] = 1.0 / {rates[currency["CurrencyCode"]]!r};']
lines += ['}']
outputs={ROOT / 'native/engine_resources.generated.h':'\n'.join(lines)+'\n'}

enum=(UP/'Calculator.ViewModels/Common/NumbersAndOperatorsEnum.cs').read_text(encoding="utf-8")
commands=re.findall(r'^        (\w+) = (\d+),',enum,re.M)
commands=[(k,int(v)) for k,v in commands if k not in ['BINSTART','BINEND','BitflipButton','FullKeypadButton']]
# MemoryStore is a manager action, not the UI's unnumbered Memory enum member.
commands += [('MemoryStore',124)]
rust=['// Generated from NumbersAndOperatorsEnum.cs; values are original native command IDs.',
      '#[derive(Clone, Copy, Debug, PartialEq, Eq)]', 'pub enum Command {']
rust += [f'    {k},' for k,v in commands]
rust += ['}', 'impl Command {', '    pub fn id(self) -> i32 { match self {']
rust += [f'        Self::{k} => {v},' for k,v in commands]
rust += ['    }}', '    pub fn from_name(name: &str) -> Option<Self> { Some(match name {']
rust += [f'        "{k}" => Self::{k},' for k,v in commands]
rust += ['        _ => return None,', '    })}', '}']
formatted=subprocess.run(['rustfmt','--edition','2024'],input='\n'.join(rust)+'\n',text=True,capture_output=True,check=True).stdout
outputs[ROOT/'src/engine_commands.rs']=formatted
for path,contents in outputs.items():
    if '--check' in sys.argv:
        if not path.exists() or path.read_text(encoding="utf-8")!=contents:raise SystemExit('Generated file differs: '+str(path))
    else:path.write_text(contents, encoding="utf-8")
print(f'Extracted {len(resources)} resources, {len(commands)} commands and original conversion data.')
