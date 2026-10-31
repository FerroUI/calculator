// Platform includes only. The upstream arithmetic sources are compiled unchanged.
#pragma once
#include <algorithm>
#include <array>
#include <cassert>
#include <cstdint>
#include <climits>
#include <cmath>
#include <cwctype>
#include <functional>
#include <iomanip>
#include <limits>
#include <list>
#include <memory>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <vector>
#if defined(__wasm__) && !defined(MIDL_PASS)
// Ratpack/conv.cpp lists the architectures it knows, which predate WebAssembly; MIDL_PASS selects
// its portable 64-bit product. The macro has no other use in the compiled sources.
#define MIDL_PASS 1
#endif
#include "sal_cross_platform.h"
#include "winerror_cross_platform.h"
