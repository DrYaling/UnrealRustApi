cargo build --release

copy .\target\release\unreal_rs.dll ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\target\release\unreal_rs.dll.lib ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\target\release\unreal_rs.pdb ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\Runtime\core_api.h ..\UnrealRustInterface\Plugins\unreal_rs\Source\unreal_rs\Public

copy .\Runtime\type_wrapper.h ..\UnrealRustInterface\Plugins\unreal_rs\Source\unreal_rs\Public
