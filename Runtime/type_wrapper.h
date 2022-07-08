#pragma once
/*
rust 基本数据类型
*/
#include <stdint.h>
struct Position{
    int32_t x;
    int32_t y;
};
typedef void(*UELogCallback)(const char* log, int32_t log_level);
typedef void* CVoidPtr;

#define push_integers(L, ptr, len, tp, cap)\
{\
	lua_createtable(L, 0, 0); \
	lua_pushnumber(L, -1); \
	lua_rawseti(L, -2, 0); \
	if (len >= 0) {\
        tp* array = (tp*)ptr;\
        for (size_t i = 0; i < len; i++)\
        {\
            lua_pushinteger(L, array[i]); \
            lua_rawseti(L, -2, i + 1); \
        }\
		ReleaseVector((unsigned char*)ptr, len, sizeof(tp), cap);\
	}\
}
#define to_array(L, pos, tp, out)\
tp* out = nullptr;\
{\
    int size = tolua_objlen(L, pos);\
    auto top = lua_gettop(L);\
    out = (tp*)malloc(size * sizeof(tp));\
    top ++;\
    for (size_t i = 1; i <= size; i++)\
    {\
		lua_rawgeti(L, pos, i);\
		out[i - 1] = lua_tointeger(L, top);\
		lua_pop(L, 1);\
    }\
    lua_settop(L, pos + 1);\
}