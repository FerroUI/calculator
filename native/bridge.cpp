// FerroCalc platform adapter. Original CalcManager, CEngine and Ratpack are unchanged.
#include "compat/portable.h"
#include "CalculatorManager.h"
#include "CalculatorResource.h"
#include "UnitConverter.h"
#include "engine_resources.generated.h"

using namespace CalculationManager;
namespace {
std::string json(std::wstring_view text) {
    std::string out = "\"";
    for (size_t i = 0; i < text.size(); ++i) {
        uint32_t c = static_cast<uint32_t>(text[i]);
        if (sizeof(wchar_t) == 2 && c >= 0xd800 && c <= 0xdbff && i+1 < text.size()) {
            c = 0x10000 + ((c - 0xd800) << 10) + (static_cast<uint32_t>(text[++i]) - 0xdc00);
        }
        if (c == '"' || c == '\\') { out += '\\'; out += static_cast<char>(c); }
        else if (c < 32) {
            const char* hex = "0123456789abcdef";
            out += "\\u00"; out += hex[c >> 4]; out += hex[c & 15];
        } else if (c < 0x80) out += static_cast<char>(c);
        else if (c < 0x800) { out += static_cast<char>(0xc0 | (c >> 6)); out += static_cast<char>(0x80 | (c & 63)); }
        else if (c < 0x10000) { out += static_cast<char>(0xe0 | (c >> 12)); out += static_cast<char>(0x80 | ((c >> 6) & 63)); out += static_cast<char>(0x80 | (c & 63)); }
        else { out += static_cast<char>(0xf0 | (c >> 18)); out += static_cast<char>(0x80 | ((c >> 12) & 63)); out += static_cast<char>(0x80 | ((c >> 6) & 63)); out += static_cast<char>(0x80 | (c & 63)); }
    }
    return out + '"';
}
struct Resources final : IResourceProvider {
    std::wstring GetCEngineString(std::wstring_view id) override {
        auto found = engine_resources.find(std::wstring(id));
        return found == engine_resources.end() ? std::wstring{} : found->second;
    }
};
struct Display final : ICalcDisplay {
    std::wstring primary = L"0", expression;
    bool error = false;
    unsigned parens = 0;
    std::vector<std::wstring> memory;
    void SetPrimaryDisplay(const std::wstring& s, bool e) override { primary=s; error=e; }
    void SetIsInError(bool e) override { error=e; }
    void SetExpressionDisplay(std::shared_ptr<std::vector<std::pair<std::wstring,int>>> const& tokens,
        std::shared_ptr<std::vector<std::shared_ptr<IExpressionCommand>>> const&) override {
        expression.clear();
        if (tokens) for (const auto& token : *tokens) expression += token.first;
    }
    void SetParenthesisNumber(unsigned n) override { parens=n; }
    void SetMemorizedNumbers(const std::vector<std::wstring>& v) override { memory=v; }
    void OnNoRightParenAdded() override {}
    void MaxDigitsReached() override {}
    void BinaryOperatorReceived() override {}
    void OnHistoryItemAdded(unsigned) override {}
    void MemoryItemChanged(unsigned) override {}
    void InputChanged() override {}
};

struct UnitLoader : UnitConversionManager::IConverterDataLoader {
    std::vector<UnitConversionManager::Category> categories;
    std::unordered_map<int,std::vector<UnitConversionManager::Unit>> units;
    std::unordered_map<int,double> factors;
    std::unordered_map<int,std::unordered_map<int,UnitConversionManager::ConversionData>> explicit_ratios;
    UnitLoader() { populate_units(categories, units, factors, explicit_ratios); }
    void LoadData() override {}
    std::vector<UnitConversionManager::Category> GetOrderedCategories() override { return categories; }
    std::vector<UnitConversionManager::Unit> GetOrderedUnits(const UnitConversionManager::Category& c) override { return units.at(c.id); }
    bool SupportsCategory(const UnitConversionManager::Category& c) override { return units.count(c.id) != 0; }
    std::unordered_map<UnitConversionManager::Unit,UnitConversionManager::ConversionData,UnitConversionManager::UnitHash>
    LoadOrderedRatios(const UnitConversionManager::Unit& source) override {
        std::unordered_map<UnitConversionManager::Unit,UnitConversionManager::ConversionData,UnitConversionManager::UnitHash> result;
        for (const auto& category : units) {
            if (std::none_of(category.second.begin(),category.second.end(),[&](const auto& u){return u.id == source.id;})) continue;
            for (const auto& target : category.second) {
                if (explicit_ratios.count(source.id)) result.emplace(target,explicit_ratios.at(source.id).at(target.id));
                else result.emplace(target,UnitConversionManager::ConversionData(factors.at(source.id)/factors.at(target.id),0,false));
            }
        }
        return result;
    }
};
struct MockCurrencyLoader final : UnitLoader {
    bool SupportsCategory(const UnitConversionManager::Category& category) override { return category.id == 0; }
};
struct UnitDisplay final : UnitConversionManager::IUnitConverterVMCallback {
    std::wstring from=L"0", to=L"0";
    std::vector<std::pair<std::wstring,std::wstring>> suggested;
    void DisplayCallback(const std::wstring& f,const std::wstring& t) override { from=f; to=t; }
    void SuggestedValueCallback(const std::vector<std::tuple<std::wstring,UnitConversionManager::Unit>>& values) override {
        suggested.clear();
        for(const auto& v:values) suggested.emplace_back(std::get<0>(v),std::get<1>(v).name);
    }
    void MaxDigitsReached() override {}
};

struct Context {
    Resources resources;
    Display display;
    CalculatorManager manager{&display,&resources};
    bool programmer=false;
    bool scientific=false;
    std::shared_ptr<UnitLoader> loader=std::make_shared<UnitLoader>();
    std::shared_ptr<UnitDisplay> unit_display=std::make_shared<UnitDisplay>();
    std::shared_ptr<UnitConversionManager::UnitConverter> converter=std::make_shared<UnitConversionManager::UnitConverter>(loader,std::make_shared<MockCurrencyLoader>());
    std::vector<UnitConversionManager::Unit> current_units;
    int from_id=0,to_id=0;
    std::string snapshot;
    Context() {
        manager.SetStandardMode();
        converter->SetViewModelCallback(unit_display);
        converter->Initialize();
        select_category(1);
    }
    void select_category(int id) {
        auto found=std::find_if(loader->categories.begin(),loader->categories.end(),[id](const auto& c){return c.id==id;});
        if(found==loader->categories.end()) throw std::invalid_argument("Unknown converter category");
        auto init=converter->SetCurrentCategory(*found);
        current_units=std::get<0>(init);
        from_id=std::get<1>(init).id; to_id=std::get<2>(init).id;
        // SetCurrentCategory selects units; its view-model client must commit them to
        // refresh the returned display and supplementary values (as upstream does).
        converter->SetCurrentUnitTypes(std::get<1>(init),std::get<2>(init));
    }
    std::string state() {
        std::string s="{\"display\":"+json(display.primary)+",\"expression\":"+json(display.expression)+",\"error\":"+(display.error?"true":"false")+",\"parentheses\":"+std::to_string(display.parens)+",\"memory\":[";
        bool comma=false;
        for(const auto& v:display.memory) {if(comma)s+=',';comma=true;s+=json(v);}
        s+="],\"history\":[";comma=false;
        if(!programmer) for(const auto& h:manager.GetHistoryItems()) {
            if(comma)s+=',';comma=true;
            s+="{\"expression\":"+json(h->historyItemVector.expression)+",\"result\":"+json(h->historyItemVector.result)+"}";
        }
        s+="],\"radices\":[";
        for(int radix:{16,10,8,2}) { if(radix!=16)s+=',';s+=json(programmer&&!display.error?manager.GetResultForRadix(radix,64,false):L""); }
        s+="],\"converter\":{\"from\":"+json(unit_display->from)+",\"to\":"+json(unit_display->to)+",\"from_id\":"+std::to_string(from_id)+",\"to_id\":"+std::to_string(to_id)+",\"categories\":[";
        comma=false;
        for(const auto& c:loader->categories){if(comma)s+=',';comma=true;s+="{\"id\":"+std::to_string(c.id)+",\"name\":"+json(c.name)+"}";}
        s+="],\"units\":[";comma=false;
        for(const auto& u:current_units) { if(u.isWhimsical)continue; if(comma)s+=',';comma=true;s+="{\"id\":"+std::to_string(u.id)+",\"name\":"+json(u.name)+"}"; }
        s+="],\"suggested\":[";comma=false;
        if(converter->GetCurrentCategory().id!=0)for(const auto& v:unit_display->suggested){if(comma)s+=',';comma=true;s+=json(v.first+L" "+v.second);}
        return s+"]}}";
    }
};
}

// All exceptions stop at the ABI. Rust serializes access to the upstream global constants.
extern "C" void* ferro_calc_create() noexcept { try{return new Context;}catch(...){return nullptr;} }
extern "C" void ferro_calc_destroy(void* p) noexcept { try{delete static_cast<Context*>(p);}catch(...){} }
extern "C" const char* ferro_calc_snapshot(void* p) noexcept {
    try{auto& c=*static_cast<Context*>(p);c.snapshot=c.state();return c.snapshot.c_str();}catch(...){return nullptr;}
}
extern "C" int ferro_calc_action(void* p,int action,int a,int b) noexcept {
    try {
        auto& c=*static_cast<Context*>(p);
        auto& m=c.manager;
        switch(action) {
        case 0:
            if(a==200||a==201||a==209){c.programmer=a==209;c.scientific=a==201;}
            m.SendCommand(static_cast<Command>(a));break;
        case 1:m.MemorizeNumber();break;
        case 2:case 3:case 4:case 5:
            if(a<0||(static_cast<size_t>(a)>=c.display.memory.size() && !((action==3||action==4)&&a==0&&c.display.memory.empty())))return 1;
            if(action==2)m.MemorizedNumberLoad(a);
            if(action==3)m.MemorizedNumberAdd(a);
            if(action==4)m.MemorizedNumberSubtract(a);
            if(action==5){m.MemorizedNumberClear(a);m.SetMemorizedNumbersString();}
            break;
        case 6:m.MemorizedNumberClearAll();break;
        case 7:if(!c.programmer)m.ClearHistory();break;
        case 8:if(c.programmer||a<0||static_cast<size_t>(a)>=m.GetHistoryItems().size())return 1;
            m.RemoveHistoryItem(a);break;
        case 9: {
            if(c.programmer||a<0||static_cast<size_t>(a)>=m.GetHistoryItems().size())return 1;
            auto item=m.GetHistoryItem(a);
            auto result=item->historyItemVector.result;
            // Restore the original expression command stream, including unary/parentheses.
            auto commands=*item->historyItemVector.spCommands;
            const auto degree=m.GetCurrentDegreeMode();
            m.SetInHistoryItemLoadMode(true);
            struct ResetFlag { CalculatorManager& m; ~ResetFlag(){m.SetInHistoryItemLoadMode(false);} } reset{m};
            m.Reset(false);
            if(c.scientific)m.SendCommand(Command::ModeScientific);
            m.SendCommand(degree);
            for(const auto& command:commands) {
                if(auto operand=std::dynamic_pointer_cast<IOpndCommand>(command)) {
                    bool need_sign=operand->IsNegative();
                    for(int n:*operand->GetCommands()) {
                        m.SendCommand(static_cast<Command>(n));
                        if(need_sign&&n!=static_cast<int>(Command::Command0)){m.SendCommand(Command::CommandSIGN);need_sign=false;}
                    }
                } else if(auto unary=std::dynamic_pointer_cast<IUnaryCommand>(command)) {
                    for(int n:*unary->GetCommands())m.SendCommand(static_cast<Command>(n));
                } else if(auto binary=std::dynamic_pointer_cast<IBinaryCommand>(command)) m.SendCommand(static_cast<Command>(binary->GetCommand()));
                else if(auto paren=std::dynamic_pointer_cast<IParenthesisCommand>(command))m.SendCommand(static_cast<Command>(paren->GetCommand()));
            }
            m.SendCommand(Command::CommandEQU);
            // The upstream view model restores these itself: Manager deliberately suppresses
            // display callbacks while a history item is replayed.
            c.display.SetPrimaryDisplay(result,false);
            c.display.expression=item->historyItemVector.expression;
            break;
        }
        case 10:c.select_category(a);break;
        case 11: {
            const auto& units=c.current_units;
            auto f=std::find_if(units.begin(),units.end(),[a](const auto& u){return u.id==a&&!u.isWhimsical;});
            auto t=std::find_if(units.begin(),units.end(),[b](const auto& u){return u.id==b&&!u.isWhimsical;});
            if(f==units.end()||t==units.end())return 1;
            c.from_id=a;c.to_id=b;c.converter->SetCurrentUnitTypes(*f,*t);break;
        }
        case 12:if(a<0||a>14)return 1;c.converter->SendCommand(static_cast<UnitConversionManager::Command>(a));break;
        case 13:c.converter->SwitchActive(c.unit_display->to);std::swap(c.from_id,c.to_id);c.converter->Calculate();break;
        default:return 1;
        }
        return 0;
    } catch(...) { return 2; }
}
