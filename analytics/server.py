from bottle import post, run, request

@post('/work')
def get_new_work():
    print(request.json)

    # take json from request
    # turn the list into a dict
    # turn dict into pandas dataframe
    # send to MS
    # return success=happy
    return "Hello world!"

run(host="localhost", port=8080, debug=True)
