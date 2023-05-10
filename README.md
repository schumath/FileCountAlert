# FileCountAlert
A small Windows App build with Rust that checks the number of files and directories in a directory (not recursive) and if there are to many files it performs an http get request. This can be used e.g. as an alert.

The config is done in the config.yml. This file has to be in the same folder as the file_count_alert.exe file. After execution the program stopps automatically. To perform the check again you have to execute it again.

## Settings:
- **path**: local path of the directory you want to monitor
- **file_limit**: the maximum number of files in the directory without sending the http request
- **get_url**: a http or https url you want to call. The GET method is used. The number of counted files/directories is added to the url like e.g. "&file_count=17"


## Building
To build a release version (on Windows) you can run the build.bat script