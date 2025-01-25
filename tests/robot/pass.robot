*** Settings ***
Library             RequestsLibrary

Suite Setup         Create HTTP Session
Suite Teardown      Teardown HTTP Session


*** Variables ***
${BASE_URL}         http://127.0.0.1:8000
${JSON_PAYLOAD}     {"array1": [2,3], "array2": [3,4]}


*** Test Cases ***
Test REST /python
    [Documentation]    curl 127.0.0.1:8000/python
    ${response}    GET On Session    mysession    /python
    Should Be Equal As Numbers    ${response.status_code}    200

Test REST /rust
    [Documentation]    curl 127.0.0.1:8000/rust
    ${response}    GET On Session    mysession    /rust
    Should Be Equal As Numbers    ${response.status_code}    200

Test REST /rust/calculator/counter
    [Documentation]    curl 127.0.0.1:8000/rust/calculator/counter
    ${response}    GET On Session    mysession    /rust/calculator/counter
    Should Be Equal As Numbers    ${response.status_code}    200

Test REST /rust/calculator/sum_array
    [Documentation]    curl 127.0.0.1:8000/rust/calculator/sum_array -d '{"array1": [2,3], "array2": [3,4]}'    -H 'Content-Type: application/json'
    ${headers}    Create Dictionary    Content-Type=application/json
    ${response}    POST On Session
    ...    mysession
    ...    /rust/calculator/sum_array
    ...    data=${JSON_PAYLOAD}
    ...    headers=${headers}
    Should Be Equal As Numbers    ${response.status_code}    200


*** Keywords ***
Create HTTP Session
    [Documentation]    Create an HTTP session before the test suite runs
    Create Session    mysession    ${BASE_URL}

Teardown HTTP Session
    [Documentation]    Delete the HTTP session after the test suite runs
    Delete All Sessions
