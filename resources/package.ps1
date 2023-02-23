param (
    [Parameter(Mandatory=$true)]
    [string]$stagingDir
)

cargo build --release


mkdir -p $stagingDir/bin/x64/plugins/
cd $stagingDir

cp ../target/release/cybercmd.dll ./bin/x64/plugins/cybercmd.asi
7z a -mx=9 -r ../cybercmd.zip *

curl https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/global.ini -O ./bin/x64/global.ini
curl https://raw.githubusercontent.com/yamashi/CyberEngineTweaks/master/vendor/asiloader/version.dll -O ./bin/x64/version.dll
7z a -mx=9 -r ../cybercmd-standalone.zip *

cd ..
