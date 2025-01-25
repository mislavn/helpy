*** Settings ***
Library             RequestsLibrary

Suite Setup         Create HTTP Session
Suite Teardown      Teardown HTTP Session


*** Variables ***
${BASE_URL}         http://127.0.0.1:8000
${JSON_PAYLOAD}     {"array1": [2], "array2": [3,4]}


*** Test Cases ***
Test failed REST /rust/calculator/sum_array
    [Documentation]    curl 127.0.0.1:8000/rust/calculator/sum_array -d '{"array1": [2], "array2": [3,4]}'    -H 'Content-Type: application/json'
    ${headers}=    Create Dictionary    Content-Type=application/json
    TRY
        ${response}=    POST On Session
        ...    mysession
        ...    /rust/calculator/sum_array
        ...    data=${JSON_PAYLOAD}
        ...    headers=${headers}
        Fail    The command should have failed
    EXCEPT
        No Operation
    END

Test failed REST /rust/error
    [Documentation]    curl 127.0.0.1:8000/rust/error
    TRY
        ${response}=    GET On Session    mysession    /rust/error
        Fail    The command should have failed
    EXCEPT
        No Operation
    END


*** Keywords ***
Create HTTP Session
    [Documentation]    Create an HTTP session before the test suite runs
    Create Session    mysession    ${BASE_URL}

Teardown HTTP Session
    [Documentation]    Delete the HTTP session after the test suite runs
    Delete All Sessions
