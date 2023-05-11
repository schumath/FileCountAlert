# FileCountAlert
A small Windows App build with Rust that checks the number of files and directories in a directory (not recursive) and if there are to many files it performs an http post request to a MS Teams webhook. This can be used e.g. as an alert.

The config is done in the config.yml. This file has to be in the same folder as the file_count_alert.exe file. After execution the program stopps automatically. To perform the check again you have to execute it again.

## Settings:
- **path**: local path of the directory you want to monitor
- **file_limit**: the maximum number of files in the directory without sending the http request
- **get_url**: a url of the MS Teams Webhook you want to call.


## Building
To build a release version (on Windows) you can run the build.bat script
