# FileCountAlert
A small Windows App build with Rust that performs an http get request in case of to many files in one directory.

The config is done in the config.yml. This file has to be in the same folder as the .exe file

Settings:
path: local path of the directory you want to monitor
file_limit: the maximum number of files in the directory without sending the http request
get_url: a http or https url you want to call. The GET method is used

