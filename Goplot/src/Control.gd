extends Control


func _ready():
    var http_request = HTTPRequest.new()
    add_child(http_request)
    http_request.connect("request_completed", self, "_http_request_completed")

    print("Sending request")
    var error = http_request.request("http://localhost:8000")
    if error != OK:
        push_error("An error occurred in the HTTP request.")


func _http_request_completed(result, response_code, headers, body):
    print("Received response of size %.3f MB" % (float(len(body)) / 1024 / 1024))
    var response = parse_json(body.get_string_from_utf8())
    var data = response["data"]
    print(len(data))
