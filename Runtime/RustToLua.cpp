/*
auto generated c header, do not modify this
*/
#include "../core_api.h"
#include "tolua.h"
int CaReleaseVector(lua_State L){
	CheckArgsCount(L, 4);
	CheckLuaType(L, 1, LUA_TTABLE);
	to_array(L, 1, unsigned char, ptr);
	CheckLuaType(L, 2, LUA_TNUMBER);
	const auto size = lua_tointeger(L, 2);
	CheckLuaType(L, 3, LUA_TNUMBER);
	const auto type_size = lua_tointeger(L, 3);
	CheckLuaType(L, 4, LUA_TNUMBER);
	const auto cap = lua_tointeger(L, 4);
	ReleaseVector((unsigned char*)ptr, (uint32_t)size, (uint32_t)type_size, (uint32_t)cap);
	if (ptr) delete[] ptr;
	return 1;
}
int CaSetRustLogLevel(lua_State L){
	CheckArgsCount(L, 1);
	CheckLuaType(L, 1, LUA_TNUMBER);
	const auto log_level = lua_tointeger(L, 1);
	const auto call_result = SetRustLogLevel((int32_t)log_level);
	lua_pushboolean(L, call_result);
	return 1;
}
int CaTestRustLog(lua_State L){
	CheckArgsCount(L, 1);
	CheckLuaType(L, 1, LUA_TNUMBER);
	const auto level = lua_tointeger(L, 1);
	TestRustLog((int32_t)level);
	return 1;
}
void InitRust2Lua(lua_State L){
  BeginClass(L, nullptr);
  RegisterFunc(L, nullptr, "CaReleaseVector", &CaReleaseVector);
  RegisterFunc(L, nullptr, "CaSetRustLogLevel", &CaSetRustLogLevel);
  RegisterFunc(L, nullptr, "CaTestRustLog", &CaTestRustLog);
  EndClass(L, nullptr);
}