#!/usr/bin/env python3
"""Reproducibly adapt original keypad trees. Every removed member is inventoried.

Run with --check in CI to detect source drift. Original XAML lives in the pinned submodule.
The generated views retain the original grids, coordinates, names, button IDs and glyphs.
"""
from pathlib import Path
import copy
import json
import re
import sys
import xml.etree.ElementTree as E

ROOT=Path(__file__).resolve().parents[1]
UP=ROOT/'upstream/calculator/src/Calculator'
X='http://schemas.microsoft.com/winfx/2006/xaml'
F='https://github.com/ferroui'
E.register_namespace('',F)
E.register_namespace('x',X)
key='{'+X+'}Key'
name='{'+X+'}Name'
def local(value):return value.split('}')[-1]
app=E.parse(UP/'App.xaml').getroot()
values={e.attrib[key]:e.text for e in app.iter() if key in e.attrib and e.text and len(e)==0}
strings={d.attrib['name']:d.findtext('value') or '' for d in E.parse(UP/'Resources/en-US/Resources.resw').getroot().findall('data')}
changes=[]
metadata={}
allowed={'UserControl','Grid','Grid.RowDefinitions','Grid.ColumnDefinitions','RowDefinition','ColumnDefinition',
         'Button','ToggleButton','RadioButton','StackPanel','Border','TextBlock','Flyout','Button.Flyout','ContentControl'}
attrs=set('Grid.Row Grid.Column Grid.RowSpan Grid.ColumnSpan Width Height MinWidth MinHeight MaxWidth MaxHeight Margin Padding HorizontalAlignment VerticalAlignment HorizontalContentAlignment VerticalContentAlignment Orientation Spacing FontFamily FontSize FontWeight Content Text Background Foreground BorderBrush BorderThickness CornerRadius FlowDirection IsVisible IsEnabled'.split())
types={'CalculatorButton':'Button','NumberPad':'ContentControl','OperatorPanelListView':'StackPanel',
       'OperatorPanelButton':'Button','OperatorPanelButton.FlyoutMenu':'Button.Flyout'}
def record(file,path,kind,old,new=None,gap='XAML-01'):
    changes.append(dict(file=file,element=path,kind=kind,original=old,replacement=new,gap=gap))
def transform(e,file,path=''):
    t=local(e.tag); n=e.attrib.get(name,t); path=path+'/'+n
    if t=='VisualStateManager.VisualStateGroups':
        # Main now implements VSM setters. Keep the original error groups verbatim;
        # layout/storyboard groups still depend on application-specific members.
        out=E.Element('{'+F+'}'+t)
        for group in e:
            if group.attrib.get(name)=='ErrorVisualStates':
                restored=copy.deepcopy(group)
                for member in restored.iter():member.tag='{'+F+'}'+local(member.tag)
                out.append(restored)
                record(file,path,'restored-subtree',E.tostring(group,encoding='unicode'),'native VisualStateManager',gap='XAML-02')
            else:record(file,path,'deferred-subtree',E.tostring(group,encoding='unicode'),gap='XAML-02')
        return out if len(out) else None
    if t.endswith('.Resources'):
        record(file,path,'adapted-resources',E.tostring(e,encoding='unicode'),gap='XAML-03');return None
    target=types.get(t,t)
    if target not in allowed:
        record(file,path,'deferred-subtree',E.tostring(e,encoding='unicode'));return None
    out=E.Element('{'+F+'}'+target)
    if target!=t:record(file,path,'type',t,target)
    if target in ['Button','ToggleButton','RadioButton']:
        original_style=e.attrib.get('Style','')
        classes=['calc']
        if 'Numeric' in original_style or file=='NumberPad.xaml':classes+=['numeric']
        if 'Symbol' in original_style or 'AccentEmphasized' in original_style:classes+=['symbol']
        if 'AccentEmphasized' in original_style:classes+=['equals']
        if 'Caption' in original_style or t=='OperatorPanelButton' or target=='RadioButton':classes+=['caption']
        out.set('Classes',' '.join(classes))
        if t=='CalculatorButton':
            command=e.attrib.get('ButtonId')
            if command:
                out.set('Tag',command)
                shortcuts={}
                for attr,value in e.attrib.items():
                    if 'KeyboardShortcutManager.' not in attr:continue
                    kind=attr.split('KeyboardShortcutManager.',1)[1]
                    if value.startswith('{utils:Resource'):
                        match=re.search(r'Name=(.*?)\}',value)
                        value=strings.get(match[1].replace('/','.'),'') if match else ''
                    if value:shortcuts[kind]=value
                metadata.setdefault(file,[]).append({'name':n,'command':command,'shortcuts':shortcuts})
            if command in ['Zero','One','Two','Three','Four','Five','Six','Seven','Eight','Nine','Decimal'] and 'Content' not in e.attrib:
                out.set('Content',str(['Zero','One','Two','Three','Four','Five','Six','Seven','Eight','Nine'].index(command)) if command!='Decimal' else '.')
    if t=='OperatorPanelListView':out.set('Orientation','Horizontal')
    for k,v in e.attrib.items():
        prop=local(k)
        if k==name:
            if target in ['RowDefinition','ColumnDefinition','Flyout']:
                record(file,path,'name',v,'indexed definition / owning button',gap='XAML-04')
            else:out.set(k,v)
            continue
        if prop=='AutomationId':out.set('AutomationProperties.AutomationId',v);continue
        if prop.startswith('AutomationProperties.'):
            if prop=='AutomationProperties.HeadingLevel':record(file,path,'attribute',k+'='+v,gap='A11Y-01');continue
        if prop=='Style':record(file,path,'style',v,'Classes='+out.attrib.get('Classes',''),gap='XAML-03');continue
        if prop=='Visibility':
            if v in ['Visible','Collapsed']:out.set('IsVisible','True' if v=='Visible' else 'False')
            else:record(file,path,'binding',k+'='+v,'Rust view update',gap='XAML-01')
            continue
        if prop=='ButtonId':record(file,path,'attribute',k+'='+v,'Tag='+v);continue
        if prop in ['Background','Foreground','BorderBrush'] and v.startswith('{'):
            record(file,path,'attribute',k+'='+v,'Calculator styles',gap='XAML-03');continue
        if prop.startswith('AutomationProperties.') and prop!='AutomationProperties.Name' and prop!='AutomationProperties.AutomationId':
            record(file,path,'attribute',k+'='+v,gap='A11Y-01');continue
        if prop=='Text' and target=='Button':continue
        if prop=='Glyph' and t=='OperatorPanelButton':continue
        if prop=='FontFamily' and 'CalculatorFontFamily' in v:
            out.set(prop,'fonts:Calculator#Calculator Fluent Icons');continue
        if prop in attrs or prop in ['AutomationProperties.Name','AutomationProperties.AutomationId']:
            if v.startswith('{utils:ResourceString Name='):
                resource=v[len('{utils:ResourceString Name='):-1].replace('/', '.')
                if resource in strings:out.set(prop,strings[resource]);continue
                record(file,path,'missing-resource',v,gap='I18N-01');continue
            if v.startswith('{StaticResource ') or v.startswith('{ThemeResource '):
                resource=v.split(' ',1)[1][:-1]
                if resource in values and not values[resource].startswith('ms-appx:'):
                    out.set(prop,values[resource].strip());continue
            if v.startswith('{'):
                record(file,path,'binding',k+'='+v,'Rust view update',gap='XAML-01');continue
            out.set(prop,v);continue
        record(file,path,'attribute',k+'='+v,gap='XAML-01')
    # The panel's localized label is stored in a WinUI-specific plain property. The content
    # follows App.xaml's OperatorPanelButtonStyle template: the panel glyph, the label and the
    # chevron (E70D), both glyphs from CalculatorIcons.
    if t=='OperatorPanelButton':
        for prop in ['Content','Text','Label']:
            value=e.attrib.get(prop,'')
            if value.startswith('{utils:ResourceString Name='):
                value=strings.get(value[len('{utils:ResourceString Name='):-1].replace('/', '.'),'')
            if value:
                panel=E.SubElement(out,'{'+F+'}StackPanel',{'Orientation':'Horizontal'})
                icons='fonts:Calculator#Calculator Fluent Icons'
                if e.attrib.get('Glyph'):
                    E.SubElement(panel,'{'+F+'}FontIcon',{'Margin':'0,2,8,0','VerticalAlignment':'Center','FontFamily':icons,'Glyph':e.attrib['Glyph']})
                E.SubElement(panel,'{'+F+'}TextBlock',{'VerticalAlignment':'Center','Text':value})
                E.SubElement(panel,'{'+F+'}FontIcon',{'Margin':'8,4,0,0','VerticalAlignment':'Center','FontFamily':icons,'Glyph':'\ue70d'})
                break
    for c in e:
        child=transform(c,file,path)
        if child is not None:out.append(child)
    if e.text and e.text.strip():out.text=e.text
    if target in ['Grid','StackPanel'] and 'Padding' in out.attrib:
        padding=out.attrib.pop('Padding')
        wrapper=E.Element('{'+F+'}Border',{'Padding':padding})
        for prop in list(out.attrib):
            if prop.startswith('Grid.') or prop in ['Margin','HorizontalAlignment','VerticalAlignment','Width','Height','MinWidth','MinHeight','MaxWidth','MaxHeight']:
                wrapper.set(prop,out.attrib.pop(prop))
        wrapper.append(out)
        record(file,path,'panel-padding',padding,'Border.Padding',gap='XAML-04')
        return wrapper
    return out

outputs={}
for file in ['NumberPad.xaml','CalculatorStandardOperators.xaml','CalculatorScientificOperators.xaml',
             'CalculatorProgrammerRadixOperators.xaml','CalculatorScientificAngleButtons.xaml']:
    out=transform(E.parse(UP/'Views'/file).getroot(),file)
    E.indent(out,space='    ')
    outputs[ROOT/'src/views'/file]='<!-- Adapted from upstream/calculator/src/Calculator/Views/'+file+'; see docs/xaml-adaptations.json. -->\n'+E.tostring(out,encoding='unicode')+'\n'

outputs[ROOT/'src/views/commands.json']=json.dumps(metadata,ensure_ascii=False,indent=2)+'\n'
outputs[ROOT/'docs/xaml-adaptations.json']=json.dumps(changes,ensure_ascii=False,indent=2)+'\n'
nav_source=(ROOT/'upstream/calculator/src/Calculator.ViewModels/Common/NavCategory.cs').read_text()
nav_glyphs={title:chr(int(glyph,16)) for title,glyph in re.findall(r'FriendlyName = "([^"]+)"[^\n]*?Glyph = "\\u([0-9A-Fa-f]+)"',nav_source)}
outputs[ROOT/'src/views/navigation.json']=json.dumps(nav_glyphs,ensure_ascii=False,indent=2)+'\n'

# Preserve original material types, keys and recipe values. FerroUI main now
# implements Reveal and AcrylicBrush, including their policy-driven fallbacks.
resources=E.Element('{'+F+'}ResourceDictionary')
themes=E.SubElement(resources,'{'+F+'}ResourceDictionary.ThemeDictionaries')
source_themes=app.find('.//{*}Application.Resources/{*}ResourceDictionary/{*}ResourceDictionary.ThemeDictionaries')
for dictionary in source_themes:
    theme=dictionary.attrib.get(key)
    if theme not in ['Default','Light']:continue
    dest=E.SubElement(themes,'{'+F+'}ResourceDictionary',{key:'Dark' if theme=='Default' else theme})
    for e in dictionary:
        t=local(e.tag)
        if t not in ['SolidColorBrush','Color','StaticResource','RevealBackgroundBrush','RevealBorderBrush','AcrylicBrush']:continue
        out=copy.deepcopy(e)
        out.tag='{'+F+'}'+t
        if t in ['RevealBackgroundBrush','RevealBorderBrush']:
            out.set('TargetTheme',out.attrib.get('TargetTheme','Dark' if theme=='Default' else 'Light'))
        for k,v in list(out.attrib.items()):out.set(k,v.replace('{ThemeResource ','{DynamicResource '))
        dest.append(out)
E.indent(resources,space='    ')
outputs[ROOT/'src/styles/CalculatorResources.xaml']='<!-- Original App.xaml colours and native Reveal/Acrylic recipes; platform material policy controls fallbacks. -->\n'+E.tostring(resources,encoding='unicode')+'\n'

bad=[]
for file,text in outputs.items():
    if '--check' in sys.argv:
        if not file.exists() or file.read_text(encoding="utf-8")!=text:bad.append(str(file.relative_to(ROOT)))
    else:file.write_text(text, encoding="utf-8")
if bad:raise SystemExit('Generated files differ: '+', '.join(bad))
print(f'{len(outputs)} generated files verified' if '--check' in sys.argv else f'{len(outputs)} files generated; {len(changes)} adaptations inventoried')
