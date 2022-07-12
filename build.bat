
copy ..\UnrealRustInterface\Plugins\unreal_rs\Source\unreal_rs\Public\type_wrapper.h .\Runtime\type_wrapper.h 

cargo build --release

copy .\..\target\release\unreal_rs.dll ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\..\target\release\unreal_rs.dll.lib ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\..\target\release\unreal_rs.pdb ..\UnrealRustInterface\Plugins\unreal_rs\Source\ThirdParty\UnrealRsLibrary\Win64\

copy .\Runtime\core_api.h ..\UnrealRustInterface\Plugins\unreal_rs\Source\unreal_rs\Public

