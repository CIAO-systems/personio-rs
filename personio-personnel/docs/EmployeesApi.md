# \EmployeesApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_employees_attributes_get**](EmployeesApi.md#company_employees_attributes_get) | **GET** /company/employees/attributes | List Allowed Attributes
[**company_employees_custom_attributes_get**](EmployeesApi.md#company_employees_custom_attributes_get) | **GET** /company/employees/custom-attributes | List Allowed Custom Attributes
[**company_employees_employee_id_absences_balance_get**](EmployeesApi.md#company_employees_employee_id_absences_balance_get) | **GET** /company/employees/{employee_id}/absences/balance | Get Absence Balance for Employee
[**company_employees_employee_id_get**](EmployeesApi.md#company_employees_employee_id_get) | **GET** /company/employees/{employee_id} | Get Employee by ID
[**company_employees_employee_id_patch**](EmployeesApi.md#company_employees_employee_id_patch) | **PATCH** /company/employees/{employee_id} | Update Employee by ID
[**company_employees_employee_id_profile_picture_width_get**](EmployeesApi.md#company_employees_employee_id_profile_picture_width_get) | **GET** /company/employees/{employee_id}/profile-picture/{width} | Get Employee Profile Picture
[**company_employees_get**](EmployeesApi.md#company_employees_get) | **GET** /company/employees | List Employees
[**company_employees_post**](EmployeesApi.md#company_employees_post) | **POST** /company/employees | Create an Employee



## company_employees_attributes_get

> company_employees_attributes_get(x_personio_partner_id, x_personio_app_id)
List Allowed Attributes

Lists all the allowed atrributes per API credentials including custom (dynamic) attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_custom_attributes_get

> company_employees_custom_attributes_get(x_personio_partner_id, x_personio_app_id)
List Allowed Custom Attributes

This endpoint is an alias for `/company/employees/attributes`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_employee_id_absences_balance_get

> models::EmployeeAbsenceBalance company_employees_employee_id_absences_balance_get(employee_id, x_personio_partner_id, x_personio_app_id)
Get Absence Balance for Employee

Retrieve the absence balance for a specific employee

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Numeric `id` of the employee | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::EmployeeAbsenceBalance**](EmployeeAbsenceBalance.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_employee_id_get

> models::EmployeeResponse company_employees_employee_id_get(employee_id, x_personio_partner_id, x_personio_app_id)
Get Employee by ID

Get Employee by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Numeric `id` of the employee | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::EmployeeResponse**](EmployeeResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_employee_id_patch

> models::EmployeeUpdatedResponse company_employees_employee_id_patch(employee_id, x_personio_partner_id, x_personio_app_id, company_employees_employee_id_patch_request)
Update Employee by ID

Updates an existing employee. Note: Only the fields that are listed in the body example are updatable. Attributes that are not part of the sample request body but are present inside the request are ignored. It's not possible to update the Email field. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Numeric `id` of the employee | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**company_employees_employee_id_patch_request** | Option<[**CompanyEmployeesEmployeeIdPatchRequest**](CompanyEmployeesEmployeeIdPatchRequest.md)> |  |  |

### Return type

[**models::EmployeeUpdatedResponse**](EmployeeUpdatedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_employee_id_profile_picture_width_get

> std::path::PathBuf company_employees_employee_id_profile_picture_width_get(employee_id, width, x_personio_partner_id, x_personio_app_id)
Get Employee Profile Picture

Show employee's profile picture. If profile picture is missing, the 404 error will be thrown. The `Profile Picture` attribute has to be whitelisted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Numeric `id` of the employee | [required] |
**width** | **i32** | Width of the image. Default is original size | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_get

> models::EmployeesResponse company_employees_get(x_personio_partner_id, x_personio_app_id, limit, offset, email, attributes_left_square_bracket_right_square_bracket, updated_since)
List Employees

List Company Employees

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**limit** | Option<**i32**> | Pagination attribute to limit the number of employees returned per page. |  |[default to 10]
**offset** | Option<**i32**> | Pagination attribute to identify the first item in the collection to return. |  |[default to 0]
**email** | Option<**String**> | Find an employee with the given email address. The response is still a list, containing only the filtered employee. NOTE: when using the updated_since filter, the email filter is ignored. |  |
**attributes_left_square_bracket_right_square_bracket** | Option<[**Vec<String>**](String.md)> | A list of attributes that will be returned for the employees, ie. a projection of the employee fields and relationships. In case `updated_since` query parameter is used, this list will additionally be used to exclude the employees that had none of the provided `attributes[]` updated since `updated_since`, ie. a selection filter. |  |
**updated_since** | Option<**String**> | Filter to select and return only the employees that have been updated after `updated_since`. When it is used together with the `attributes[]` query parameter, the filter will select only the employees that have had any of the provided `attributes[]` updated since `updated_since`. The format is `ISO 8601` (2022-12-24T08:15:30) or `YYYY-MM-DD`. NOTE: when using the `updated_since` filter, the `email`, `limit`, and `offset` parameters are ignored. <details> <summary><b>Examples with <code>updated_since</code> and <code>attributes[]</code>:</b></summary><br /> In an example company that has 17 employees:<br /><br /> <details>   <summary>   <code class=\"language-html\">?updated_since=2022-12-24T08:15:30</code>   will yield 10 employees that were recently updated:   </summary> <p><pre class=\"prettyprint\"><code language=\"json\" class=\"language-json\"> {   \"success\": true,   \"metadata\": {     \"total_elements\": 10,     \"current_page\": 0,     \"total_pages\": 10   },   \"offset\": 0,   \"limit\": 1,   \"data\": [     {       \"type\": \"Employee\",       \"attributes\": {         \"id\": {           \"label\": \"ID\",           \"value\": 1,           \"type\": \"integer\",           \"universal_id\": \"id\"         },         \"first_name\": {           \"label\": \"First name\",           \"value\": \"Alexander\",           \"type\": \"standard\",           \"universal_id\": \"first_name\"         },         \"last_name\": {           \"label\": \"Last name\",           \"value\": \"Bergmann\",           \"type\": \"standard\",           \"universal_id\": \"last_name\"         },         \"email\": {           \"label\": \"Email\",           \"value\": \"alexander.bergmann@demo.com\",           \"type\": \"standard\",           \"universal_id\": \"email\"         }       }     }, ...   ],   ...   ... } </code></pre></p> </details><br /> <details>   <summary>   <code class=\"language-html\">?attributes[]=first_name</code>   will yield all 17 employees:   </summary> <p><pre class=\"prettyprint\"><code language=\"json\" class=\"language-json\"> {   \"success\": true,   \"metadata\": {     \"total_elements\": 17,     \"current_page\": 0,     \"total_pages\": 17   },   \"offset\": 0,   \"limit\": 1,   \"data\": [     {       \"type\": \"Employee\",       \"attributes\": {         \"id\": {           \"label\": \"ID\",           \"value\": 1,           \"type\": \"integer\",           \"universal_id\": \"id\"         },         \"first_name\": {           \"label\": \"First name\",           \"value\": \"Alexander\",           \"type\": \"standard\",           \"universal_id\": \"first_name\"         }       }     }   ] } </code></pre></p> </details><br /> <details>   <summary>   <code class=\"language-html\">?attributes[]=first_name&updated_since=2022-12-24T08:15:30</code>   will yield 3 employees, ones that had their <code>first_name</code> changed since <code>2022-12-24T08:15:30</code>:   </summary> <p><pre class=\"prettyprint\"><code language=\"json\" class=\"language-json\"> {   \"success\": true,   \"metadata\": {     \"total_elements\": 3,     \"current_page\": 0,     \"total_pages\": 3   },   \"offset\": 0,   \"limit\": 1,   \"data\": [     {       \"type\": \"Employee\",       \"attributes\": {         \"id\": {           \"label\": \"ID\",           \"value\": 1,           \"type\": \"integer\",           \"universal_id\": \"id\"         },         \"first_name\": {           \"label\": \"First name\",           \"value\": \"Alexander\",           \"type\": \"standard\",           \"universal_id\": \"first_name\"         }       }     }   ] } </code></pre></p> </details> </details> |  |

### Return type

[**models::EmployeesResponse**](EmployeesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_employees_post

> models::EmployeeCreatedResponse company_employees_post(x_personio_partner_id, x_personio_app_id, company_employees_post_request)
Create an Employee

Creates a new employee. If the employee's status is not provided, it will be set based on the `hire_date` value - if it is in the past, status will be `active`, otherwise `onboarding`. This endpoint responds with the `id` of the created employee in case of success. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**company_employees_post_request** | Option<[**CompanyEmployeesPostRequest**](CompanyEmployeesPostRequest.md)> |  |  |

### Return type

[**models::EmployeeCreatedResponse**](EmployeeCreatedResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

