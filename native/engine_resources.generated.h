// Generated from original Microsoft resources. See scripts/generate_engine_resources.py.
static const std::unordered_map<std::wstring, std::wstring> engine_resources = {
    {L"10", L"Rsh"},
    {L"100", L"Invalid input"},
    {L"101", L"Result is undefined"},
    {L"105", L"Not enough memory"},
    {L"107", L"Overflow"},
    {L"108", L"Result not defined"},
    {L"11", L"÷"},
    {L"118", L"Result not defined"},
    {L"119", L"Overflow"},
    {L"12", L"×"},
    {L"120", L"Overflow"},
    {L"13", L"+"},
    {L"14", L"-"},
    {L"15", L"Mod"},
    {L"16", L"yroot"},
    {L"17", L"^"},
    {L"18", L"Int"},
    {L"19", L"RoL"},
    {L"2", L"CE"},
    {L"20", L"RoR"},
    {L"21", L"NOT"},
    {L"22", L"sin"},
    {L"23", L"cis"},
    {L"24", L"tan"},
    {L"25", L"sinh"},
    {L"26", L"cosh"},
    {L"27", L"tanh"},
    {L"28", L"ln"},
    {L"29", L"log"},
    {L"30", L"√"},
    {L"35", L"dms"},
    {L"37", L"10^"},
    {L"38", L"%"},
    {L"4", L"."},
    {L"40", L"Pi"},
    {L"41", L"="},
    {L"47", L"Exp"},
    {L"48", L"("},
    {L"49", L")"},
    {L"6", L"AND"},
    {L"66", L"frac"},
    {L"67", L"sin₀"},
    {L"68", L"cos₀"},
    {L"69", L"tan₀"},
    {L"7", L"OR"},
    {L"70", L"sin₀⁻¹"},
    {L"71", L"cos₀⁻¹"},
    {L"72", L"tan₀⁻¹"},
    {L"73", L"sinᵣ"},
    {L"74", L"cosᵣ"},
    {L"75", L"tanᵣ"},
    {L"76", L"sinᵣ⁻¹"},
    {L"77", L"cosᵣ⁻¹"},
    {L"78", L"tanᵣ⁻¹"},
    {L"79", L"sin₉"},
    {L"8", L"XOR"},
    {L"80", L"cos₉"},
    {L"81", L"tan₉"},
    {L"82", L"sin₉⁻¹"},
    {L"83", L"cos₉⁻¹"},
    {L"84", L"tan₉⁻¹"},
    {L"85", L"sinh⁻¹"},
    {L"86", L"cosh⁻¹"},
    {L"87", L"tanh⁻¹"},
    {L"88", L"e^"},
    {L"89", L"10^"},
    {L"9", L"Lsh"},
    {L"90", L"√"},
    {L"91", L"sqr"},
    {L"92", L"cube"},
    {L"94", L"fact"},
    {L"95", L"1/"},
    {L"96", L"degrees"},
    {L"97", L"negate"},
    {L"99", L"Cannot divide by zero"},
    {L"SecDeg", L"sec₀"},
    {L"SecRad", L"secᵣ"},
    {L"SecGrad", L"sec₉"},
    {L"InverseSecDeg", L"sec₀⁻¹"},
    {L"InverseSecRad", L"secᵣ⁻¹"},
    {L"InverseSecGrad", L"sec₉⁻¹"},
    {L"CscDeg", L"csc₀"},
    {L"CscRad", L"cscᵣ"},
    {L"CscGrad", L"csc₉"},
    {L"InverseCscDeg", L"csc₀⁻¹"},
    {L"InverseCscRad", L"cscᵣ⁻¹"},
    {L"InverseCscGrad", L"csc₉⁻¹"},
    {L"CotDeg", L"cot₀"},
    {L"CotRad", L"cotᵣ"},
    {L"CotGrad", L"cot₉"},
    {L"InverseCotDeg", L"cot₀⁻¹"},
    {L"InverseCotRad", L"cotᵣ⁻¹"},
    {L"InverseCotGrad", L"cot₉⁻¹"},
    {L"Sech", L"sech"},
    {L"InverseSech", L"sech⁻¹"},
    {L"Csch", L"csch"},
    {L"InverseCsch", L"csch⁻¹"},
    {L"Coth", L"coth"},
    {L"InverseCoth", L"coth⁻¹"},
    {L"TwoPowX", L"2^"},
    {L"LogBaseY", L"log base"},
    {L"Abs", L"abs"},
    {L"Ceil", L"ceil"},
    {L"Floor", L"floor"},
    {L"Nand", L"NAND"},
    {L"Nor", L"NOR"},
    {L"CubeRoot", L"cuberoot"},
    {L"ProgrammerMod", L"%"},
    {L"sDecimal", L"."},
    {L"sThousand", L","},
    {L"sGrouping", L"3;0"},
};
static void populate_units(std::vector<UnitConversionManager::Category>& categories,
    std::unordered_map<int, std::vector<UnitConversionManager::Unit>>& units,
    std::unordered_map<int, double>& factors,
    std::unordered_map<int, std::unordered_map<int, UnitConversionManager::ConversionData>>& explicit_ratios) {
    categories.emplace_back(1, L"Volume", false);
    categories.emplace_back(2, L"Length", false);
    categories.emplace_back(3, L"Weight and mass", false);
    categories.emplace_back(4, L"Temperature", true);
    categories.emplace_back(5, L"Energy", false);
    categories.emplace_back(6, L"Area", false);
    categories.emplace_back(7, L"Speed", false);
    categories.emplace_back(8, L"Time", false);
    categories.emplace_back(9, L"Power", true);
    categories.emplace_back(10, L"Data", false);
    categories.emplace_back(11, L"Pressure", false);
    categories.emplace_back(12, L"Angle", true);
    units[1].emplace_back(75, L"Milliliters", L"mL", false, true, false);
    units[1].emplace_back(64, L"Cubic centimeters", L"cm³", false, false, false);
    units[1].emplace_back(74, L"Liters", L"L", false, false, false);
    units[1].emplace_back(67, L"Cubic meters", L"m³", false, false, false);
    units[1].emplace_back(79, L"Teaspoons (US)", L"tsp. (US)", true, false, false);
    units[1].emplace_back(78, L"Tablespoons (US)", L"tbsp. (US)", false, false, false);
    units[1].emplace_back(71, L"Fluid ounces (US)", L"fl oz (US)", false, false, false);
    units[1].emplace_back(69, L"Cups (US)", L"cup (US)", false, false, false);
    units[1].emplace_back(77, L"Pints (US)", L"pt (US)", false, false, false);
    units[1].emplace_back(81, L"Quarts (US)", L"qt (US)", false, false, false);
    units[1].emplace_back(73, L"Gallons (US)", L"gal (US)", false, false, false);
    units[1].emplace_back(66, L"Cubic inches", L"in³", false, false, false);
    units[1].emplace_back(65, L"Cubic feet", L"ft³", false, false, false);
    units[1].emplace_back(68, L"Cubic yards", L"yd³", false, false, false);
    units[1].emplace_back(115, L"Teaspoons (UK)", L"tsp. (UK)", false, false, false);
    units[1].emplace_back(116, L"Tablespoons (UK)", L"tbsp. (UK)", false, false, false);
    units[1].emplace_back(70, L"Fluid ounces (UK)", L"fl oz (UK)", false, false, false);
    units[1].emplace_back(76, L"Pints (UK)", L"pt (UK)", false, false, false);
    units[1].emplace_back(80, L"Quarts (UK)", L"qt (UK)", false, false, false);
    units[1].emplace_back(72, L"Gallons (UK)", L"gal (UK)", false, false, false);
    units[1].emplace_back(124, L"coffee cups", L"coffee cups", false, false, true);
    units[1].emplace_back(111, L"bathtubs", L"bathtubs", false, false, true);
    units[1].emplace_back(125, L"swimming pools", L"swimming pools", false, false, true);
    units[2].emplace_back(168, L"Angstroms", L"A", false, false, false);
    units[2].emplace_back(38, L"Nanometers", L"nm", false, false, false);
    units[2].emplace_back(35, L"Microns", L"µm", false, false, false);
    units[2].emplace_back(37, L"Millimeters", L"mm", false, false, false);
    units[2].emplace_back(30, L"Centimeters", L"cm", false, true, false);
    units[2].emplace_back(34, L"Meters", L"m", false, false, false);
    units[2].emplace_back(33, L"Kilometers", L"km", false, false, false);
    units[2].emplace_back(32, L"Inches", L"in", true, false, false);
    units[2].emplace_back(31, L"Feet", L"ft", false, false, false);
    units[2].emplace_back(40, L"Yards", L"yd", false, false, false);
    units[2].emplace_back(36, L"Miles", L"mi", false, false, false);
    units[2].emplace_back(39, L"Nautical miles", L"nmi", false, false, false);
    units[2].emplace_back(105, L"paperclips", L"paperclips", false, false, true);
    units[2].emplace_back(131, L"hands", L"hands", false, false, true);
    units[2].emplace_back(107, L"jumbo jets", L"jumbo jets", false, false, true);
    units[3].emplace_back(82, L"Carats", L"CD", false, false, false);
    units[3].emplace_back(90, L"Milligrams", L"mg", false, false, false);
    units[3].emplace_back(83, L"Centigrams", L"cg", false, false, false);
    units[3].emplace_back(84, L"Decigrams", L"dg", false, false, false);
    units[3].emplace_back(86, L"Grams", L"g", false, false, false);
    units[3].emplace_back(85, L"Dekagrams", L"dag", false, false, false);
    units[3].emplace_back(87, L"Hectograms", L"hg", false, false, false);
    units[3].emplace_back(88, L"Kilograms", L"kg", false, true, false);
    units[3].emplace_back(95, L"Metric tonnes", L"t", false, false, false);
    units[3].emplace_back(91, L"Ounces", L"oz", false, false, false);
    units[3].emplace_back(92, L"Pounds", L"lb", true, false, false);
    units[3].emplace_back(94, L"Stone", L"st", false, false, false);
    units[3].emplace_back(93, L"Short tons (US)", L"ton (US)", false, false, false);
    units[3].emplace_back(89, L"Long tons (UK)", L"ton (UK)", false, false, false);
    units[3].emplace_back(113, L"snowflakes", L"snowflakes", false, false, true);
    units[3].emplace_back(133, L"soccer balls", L"soccer balls", false, false, true);
    units[3].emplace_back(114, L"elephants", L"elephants", false, false, true);
    units[3].emplace_back(123, L"whales", L"whales", false, false, true);
    units[4].emplace_back(46, L"Celsius", L"°C", false, true, false);
    units[4].emplace_back(47, L"Fahrenheit", L"°F", true, false, false);
    units[4].emplace_back(48, L"Kelvin", L"K", false, false, false);
    units[5].emplace_back(25, L"Electron volts", L"eV", false, false, false);
    units[5].emplace_back(27, L"Joules", L"J", true, false, false);
    units[5].emplace_back(29, L"Kilojoules", L"kJ", false, false, false);
    units[5].emplace_back(24, L"Thermal calories", L"cal", false, false, false);
    units[5].emplace_back(28, L"Food calories", L"kcal", false, true, false);
    units[5].emplace_back(26, L"Foot-pounds", L"ft•lb", false, false, false);
    units[5].emplace_back(23, L"British thermal units", L"BTU", false, false, false);
    units[5].emplace_back(103, L"batteries", L"batteries", false, false, true);
    units[5].emplace_back(129, L"bananas", L"bananas", false, false, true);
    units[5].emplace_back(130, L"slices of cake", L"slices of cake", false, false, true);
    units[5].emplace_back(166, L"Kilowatt-hours", L"kWh", true, false, false);
    units[6].emplace_back(9, L"Square millimeters", L"mm²", false, false, false);
    units[6].emplace_back(3, L"Square centimeters", L"cm²", false, false, false);
    units[6].emplace_back(7, L"Square meters", L"m²", false, true, false);
    units[6].emplace_back(2, L"Hectares", L"ha", false, false, false);
    units[6].emplace_back(6, L"Square kilometers", L"km²", false, false, false);
    units[6].emplace_back(5, L"Square inches", L"in²", false, false, false);
    units[6].emplace_back(4, L"Square feet", L"ft²", true, false, false);
    units[6].emplace_back(10, L"Square yards", L"yd²", false, false, false);
    units[6].emplace_back(1, L"Acres", L"ac", false, false, false);
    units[6].emplace_back(8, L"Square miles", L"mi²", false, false, false);
    units[6].emplace_back(118, L"hands", L"hands", false, false, true);
    units[6].emplace_back(127, L"sheets of paper", L"sheets of paper", false, false, true);
    units[6].emplace_back(99, L"soccer fields", L"soccer fields", false, false, true);
    units[6].emplace_back(128, L"castles", L"castles", false, false, true);
    units[7].emplace_back(57, L"Centimeters per second", L"cm/s", false, false, false);
    units[7].emplace_back(62, L"Meters per second", L"m/s", false, false, false);
    units[7].emplace_back(59, L"Kilometers per hour", L"km/h", false, true, false);
    units[7].emplace_back(58, L"Feet per second", L"ft/s", false, false, false);
    units[7].emplace_back(63, L"Miles per hour", L"mph", true, false, false);
    units[7].emplace_back(60, L"Knots", L"kn", false, false, false);
    units[7].emplace_back(61, L"Mach", L"M", false, false, false);
    units[7].emplace_back(121, L"turtles", L"turtles", false, false, true);
    units[7].emplace_back(126, L"horses", L"horses", false, false, true);
    units[7].emplace_back(122, L"jets", L"jets", false, false, true);
    units[8].emplace_back(51, L"Microseconds", L"µs", false, false, false);
    units[8].emplace_back(52, L"Milliseconds", L"ms", false, false, false);
    units[8].emplace_back(54, L"Seconds", L"s", false, false, false);
    units[8].emplace_back(53, L"Minutes", L"min", false, true, false);
    units[8].emplace_back(50, L"Hours", L"hr", true, false, false);
    units[8].emplace_back(49, L"Days", L"d", false, false, false);
    units[8].emplace_back(55, L"Weeks", L"wk", false, false, false);
    units[8].emplace_back(56, L"Years", L"yr", false, false, false);
    units[9].emplace_back(45, L"Watts", L"W", false, false, false);
    units[9].emplace_back(44, L"Kilowatts", L"kW", true, false, false);
    units[9].emplace_back(43, L"Horsepower (US)", L"hp (US)", false, true, false);
    units[9].emplace_back(42, L"Foot-pounds/minute", L"ft•lb/min", false, false, false);
    units[9].emplace_back(41, L"BTUs/minute", L"BTU/min", false, false, false);
    units[9].emplace_back(108, L"light bulbs", L"light bulbs", false, false, true);
    units[9].emplace_back(109, L"horses", L"horses", false, false, true);
    units[9].emplace_back(132, L"train engines", L"train engines", false, false, true);
    units[10].emplace_back(11, L"Bits", L"b", false, false, false);
    units[10].emplace_back(167, L"Nibble", L"nybl", false, false, false);
    units[10].emplace_back(12, L"Bytes", L"B", false, false, false);
    units[10].emplace_back(15, L"Kilobits", L"Kb", false, false, false);
    units[10].emplace_back(149, L"Kibibits", L"Ki", false, false, false);
    units[10].emplace_back(16, L"Kilobytes", L"KB", false, false, false);
    units[10].emplace_back(150, L"Kibibytes", L"KiB", false, false, false);
    units[10].emplace_back(17, L"Megabits", L"Mb", false, false, false);
    units[10].emplace_back(151, L"Mebibits", L"Mi", false, false, false);
    units[10].emplace_back(18, L"Megabytes", L"MB", false, true, false);
    units[10].emplace_back(152, L"Mebibytes", L"MiB", false, false, false);
    units[10].emplace_back(13, L"Gigabits", L"Gb", false, false, false);
    units[10].emplace_back(147, L"Gibibits", L"Gi", false, false, false);
    units[10].emplace_back(100, L"floppy disks", L"floppy disks", false, false, true);
    units[10].emplace_back(14, L"Gigabytes", L"GB", true, false, false);
    units[10].emplace_back(101, L"CDs", L"CDs", false, false, true);
    units[10].emplace_back(148, L"Gibibytes", L"GiB", false, false, false);
    units[10].emplace_back(102, L"DVDs", L"DVDs", false, false, true);
    units[10].emplace_back(21, L"Terabits", L"Tb", false, false, false);
    units[10].emplace_back(155, L"Tebibits", L"Ti", false, false, false);
    units[10].emplace_back(22, L"Terabytes", L"TB", false, false, false);
    units[10].emplace_back(156, L"Tebibytes", L"TiB", false, false, false);
    units[10].emplace_back(19, L"Petabits", L"Pb", false, false, false);
    units[10].emplace_back(153, L"Pebibits", L"Pi", false, false, false);
    units[10].emplace_back(20, L"Petabytes", L"PB", false, false, false);
    units[10].emplace_back(154, L"Pebibytes", L"PiB", false, false, false);
    units[10].emplace_back(143, L"Exabits", L"E", false, false, false);
    units[10].emplace_back(145, L"Exbibits", L"Ei", false, false, false);
    units[10].emplace_back(144, L"Exabytes", L"EB", false, false, false);
    units[10].emplace_back(146, L"Exbibytes", L"EiB", false, false, false);
    units[10].emplace_back(163, L"Zetabits", L"Z", false, false, false);
    units[10].emplace_back(161, L"Zebibits", L"Zi", false, false, false);
    units[10].emplace_back(164, L"Zetabytes", L"ZB", false, false, false);
    units[10].emplace_back(162, L"Zebibytes", L"ZiB", false, false, false);
    units[10].emplace_back(159, L"Yottabits", L"Y", false, false, false);
    units[10].emplace_back(157, L"Yobibits", L"Yi", false, false, false);
    units[10].emplace_back(160, L"Yottabytes", L"YB", false, false, false);
    units[10].emplace_back(158, L"Yobibytes", L"YiB", false, false, false);
    units[11].emplace_back(137, L"Atmospheres", L"atm", true, false, false);
    units[11].emplace_back(138, L"Bars", L"ba", false, true, false);
    units[11].emplace_back(139, L"Kilopascals", L"kPa", false, false, false);
    units[11].emplace_back(140, L"Millimeters of mercury ", L"mmHg", false, false, false);
    units[11].emplace_back(141, L"Pascals", L"Pa", false, false, false);
    units[11].emplace_back(142, L"Pounds per square inch", L"psi", false, false, false);
    units[12].emplace_back(134, L"Degrees", L"deg", true, false, false);
    units[12].emplace_back(135, L"Radians", L"rad", false, true, false);
    units[12].emplace_back(136, L"Gradians", L"grad", false, false, false);
    factors[1] = 4046.8564224;
    factors[7] = 1;
    factors[4] = 0.09290304;
    factors[10] = 0.83612736;
    factors[9] = 0.000001;
    factors[3] = 0.0001;
    factors[5] = 0.00064516;
    factors[8] = 2589988.110336;
    factors[6] = 1000000;
    factors[2] = 10000;
    factors[118] = 0.012516104;
    factors[127] = 0.06032246;
    factors[99] = 10869.66;
    factors[128] = 100000;
    factors[11] = 0.000000125;
    factors[167] = 0.0000005;
    factors[12] = 0.000001;
    factors[16] = 0.001;
    factors[18] = 1;
    factors[14] = 1000;
    factors[22] = 1000000;
    factors[20] = 1000000000;
    factors[144] = 1000000000000;
    factors[164] = 1000000000000000;
    factors[160] = 1000000000000000000;
    factors[15] = 0.000125;
    factors[17] = 0.125;
    factors[13] = 125;
    factors[21] = 125000;
    factors[19] = 125000000;
    factors[143] = 125000000000;
    factors[163] = 125000000000000;
    factors[159] = 125000000000000000;
    factors[147] = 134.217728;
    factors[148] = 1073.741824;
    factors[149] = 0.000128;
    factors[150] = 0.001024;
    factors[151] = 0.131072;
    factors[152] = 1.048576;
    factors[153] = 140737488.355328;
    factors[154] = 1125899906.842624;
    factors[155] = 137438.953472;
    factors[156] = 1099511.627776;
    factors[145] = 144115188075.855872;
    factors[146] = 1152921504606.846976;
    factors[161] = 147573952589676.412928;
    factors[162] = 1180591620717411.303424;
    factors[157] = 151115727451828646.838272;
    factors[158] = 1208925819614629174.706176;
    factors[100] = 1.474560;
    factors[101] = 700;
    factors[102] = 4700;
    factors[24] = 4.184;
    factors[28] = 4184;
    factors[23] = 1055.056;
    factors[29] = 1000;
    factors[166] = 3600000;
    factors[25] = 0.0000000000000000001602176565;
    factors[27] = 1;
    factors[26] = 1.3558179483314;
    factors[103] = 9000;
    factors[129] = 439614;
    factors[130] = 1046700;
    factors[32] = 0.0254;
    factors[31] = 0.3048;
    factors[40] = 0.9144;
    factors[36] = 1609.344;
    factors[35] = 0.000001;
    factors[37] = 0.001;
    factors[38] = 0.000000001;
    factors[168] = 0.0000000001;
    factors[30] = 0.01;
    factors[34] = 1;
    factors[33] = 1000;
    factors[39] = 1852;
    factors[105] = 0.035052;
    factors[131] = 0.18669;
    factors[107] = 76;
    factors[41] = 17.58426666666667;
    factors[42] = 0.0225969658055233;
    factors[45] = 1;
    factors[44] = 1000;
    factors[43] = 745.69987158227022;
    factors[108] = 60;
    factors[109] = 745.7;
    factors[132] = 2982799.486329081;
    factors[49] = 86400;
    factors[54] = 1;
    factors[55] = 604800;
    factors[56] = 31557600;
    factors[52] = 0.001;
    factors[51] = 0.000001;
    factors[53] = 60;
    factors[50] = 3600;
    factors[69] = 236.588237;
    factors[77] = 473.176473;
    factors[76] = 568.26125;
    factors[81] = 946.352946;
    factors[80] = 1136.5225;
    factors[73] = 3785.411784;
    factors[72] = 4546.09;
    factors[74] = 1000;
    factors[79] = 4.92892159375;
    factors[78] = 14.78676478125;
    factors[64] = 1;
    factors[68] = 764554.857984;
    factors[67] = 1000000;
    factors[75] = 1;
    factors[66] = 16.387064;
    factors[65] = 28316.846592;
    factors[71] = 29.5735295625;
    factors[70] = 28.4130625;
    factors[115] = 5.91938802083333333333;
    factors[116] = 17.7581640625;
    factors[124] = 236.5882;
    factors[111] = 378541.2;
    factors[125] = 3750000000;
    factors[88] = 1;
    factors[87] = 0.1;
    factors[85] = 0.01;
    factors[86] = 0.001;
    factors[92] = 0.45359237;
    factors[91] = 0.028349523125;
    factors[90] = 0.000001;
    factors[83] = 0.00001;
    factors[84] = 0.0001;
    factors[89] = 1016.0469088;
    factors[95] = 1000;
    factors[94] = 6.35029318;
    factors[82] = 0.0002;
    factors[93] = 907.18474;
    factors[113] = 0.000002;
    factors[133] = 0.4325;
    factors[114] = 4000;
    factors[123] = 90000;
    factors[57] = 1;
    factors[58] = 30.48;
    factors[59] = 27.777777777777777777778;
    factors[60] = 51.44;
    factors[61] = 34030;
    factors[62] = 100;
    factors[63] = 44.7;
    factors[121] = 8.94;
    factors[126] = 2011.5;
    factors[122] = 24585;
    factors[134] = 1;
    factors[135] = 57.29577951308233;
    factors[136] = 0.9;
    factors[137] = 1;
    factors[138] = 0.9869232667160128;
    factors[139] = 0.0098692326671601;
    factors[140] = 0.0013155687145324;
    factors[141] = 9.869232667160128e-6;
    factors[142] = 0.068045961016531;
    explicit_ratios[46].emplace(46, UnitConversionManager::ConversionData(1, 0, false));
    explicit_ratios[46].emplace(47, UnitConversionManager::ConversionData(1.8, 32, false));
    explicit_ratios[46].emplace(48, UnitConversionManager::ConversionData(1, 273.15, false));
    explicit_ratios[47].emplace(46, UnitConversionManager::ConversionData(0.55555555555555555555555555555556, -32, true));
    explicit_ratios[47].emplace(47, UnitConversionManager::ConversionData(1, 0, false));
    explicit_ratios[47].emplace(48, UnitConversionManager::ConversionData(0.55555555555555555555555555555556, 459.67, true));
    explicit_ratios[48].emplace(46, UnitConversionManager::ConversionData(1, -273.15, true));
    explicit_ratios[48].emplace(47, UnitConversionManager::ConversionData(1.8, -459.67, false));
    explicit_ratios[48].emplace(48, UnitConversionManager::ConversionData(1, 0, false));
    categories.emplace_back(0, L"Currency (upstream mock rates)", false);
    units[0].emplace_back(1001, L"Mars — The Martian Dollar", L"MAR", true, false, false);
    factors[1001] = 1.0 / 1.0;
    units[0].emplace_back(1002, L"Moon — Moon Bucks", L"MON", false, true, false);
    factors[1002] = 1.0 / 0.5;
    units[0].emplace_back(1003, L"Neptune — Space Coins", L"NEP", false, false, false);
    factors[1003] = 1.0 / 0.00125;
    units[0].emplace_back(1004, L"Saturn — Rings", L"SAT", false, false, false);
    factors[1004] = 1.0 / 0.25;
    units[0].emplace_back(1005, L"Uranus — Galaxy Credits", L"URA", false, false, false);
    factors[1005] = 1.0 / 2.75;
    units[0].emplace_back(1006, L"Venus — Venusian Seashells", L"VEN", false, false, false);
    factors[1006] = 1.0 / 900.0;
    units[0].emplace_back(1007, L"Jupiter — Gas Money", L"JUP", false, false, false);
    factors[1007] = 1.0 / 1.234567891234568;
    units[0].emplace_back(1008, L"Mercury — Sun Notes", L"MER", false, false, false);
    factors[1008] = 1.0 / 2.0;
    units[0].emplace_back(1009, L"Test No Fractional Digits — Test No Fractional Digits", L"JPY", false, false, false);
    factors[1009] = 1.0 / 0.00125;
    units[0].emplace_back(1010, L"Test Fractional Digits — Test Fractional Digits", L"JOD", false, false, false);
    factors[1010] = 1.0 / 0.25;
}
