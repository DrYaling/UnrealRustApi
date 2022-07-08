#pragma once
/*
auto generated c header, do not modify this
*/
#include <stdint.h>
#include "type_wrapper.h"
typedef int32_t* I32Ptr;
typedef uint32_t* U32Ptr;
typedef const int32_t* CI32Ptr;
typedef const uint32_t* CU32Ptr;
typedef char* CharPtr;
typedef unsigned char* UCharPtr;
typedef const char* CCharPtr;
typedef unsigned const char* UCCharPtr;
extern "C"{
    void ReleaseVector(unsigned char*/*wraper of *mut u8*/ ptr, uint32_t size, uint32_t type_size, uint32_t cap);
    bool SetRustLogLevel(int32_t log_level);
    void TestRustLog(int32_t level);
    void BindDebugLogger(UELogCallback/*wraper of UELogCallback*/ func);
}