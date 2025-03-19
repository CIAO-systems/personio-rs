# \AttendancesApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_attendances_get**](AttendancesApi.md#company_attendances_get) | **GET** /company/attendances | List Attendances
[**company_attendances_id_delete**](AttendancesApi.md#company_attendances_id_delete) | **DELETE** /company/attendances/{id} | Delete Attendance by ID
[**company_attendances_id_patch**](AttendancesApi.md#company_attendances_id_patch) | **PATCH** /company/attendances/{id} | Update Attendance by ID
[**company_attendances_post**](AttendancesApi.md#company_attendances_post) | **POST** /company/attendances | Create an Attendance



## company_attendances_get

> models::AttendancePeriodsResponse company_attendances_get(start_date, end_date, x_personio_partner_id, x_personio_app_id, updated_from, updated_to, include_pending, employees_left_square_bracket_right_square_bracket, limit, offset)
List Attendances

Fetch attendance data for the company employees. The result can be `paginated` and `filtered` by period, the date and/or time they were updated, and/or specific employee/employees. The result contains a list of attendances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_date** | **String** | First day of the period to be queried. It is inclusive, so the day specified as start_date will also be considered on the results | [required] |
**end_date** | **String** | Last day of the period to be queried. It is inclusive, so the day specified as end_date will also be considered on the results. | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**updated_from** | Option<**String**> | Datetime from when the queried periods have been updated. Same format as updated_at. It is inclusive, so the day specified as updated_from will also be considered on the results. Can be just the date, or the date and the time, with or without the timezone. |  |
**updated_to** | Option<**String**> | Datetime until when the queried periods have been updated. Same format as updated_at. It is inclusive, so the day specified as updated_to will also be considered on the results. Can be just the date, or the date and the time, with or without the timezone. |  |
**include_pending** | Option<**bool**> | Returns AttendancePeriods with a status of pending, rejected and confirmed. For pending periods, the end_date attribute is nullable. The status of each period is included in the response. |  |
**employees_left_square_bracket_right_square_bracket** | Option<[**Vec<i32>**](i32.md)> | A list of Personio employee ID's to filter the results. The result filters including only attendances of provided employees. |  |
**limit** | Option<**i32**> | Pagination attribute to limit how many attendances are per page |  |[default to 200]
**offset** | Option<**i32**> | The offset from the first record that would be returned. With 3 results [A, B, C] and an offset of 1, the following two results will be returned [B, C]. |  |[default to 0]

### Return type

[**models::AttendancePeriodsResponse**](AttendancePeriodsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_id_delete

> models::DeletedAttendanceResponse company_attendances_id_delete(id, x_personio_partner_id, x_personio_app_id, skip_approval)
Delete Attendance by ID

This endpoint is responsible for deleting attendance data for the company employees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the attendance period to delete | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**skip_approval** | Option<**bool**> | Optional, default value is true. If set to false, the approval status of the attendance period will be \"pending\" if an approval rule is set for the attendances type. The respective approval flow will be triggered. |  |[default to true]

### Return type

[**models::DeletedAttendanceResponse**](DeletedAttendanceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_id_patch

> models::UpdatedAttendanceResponse company_attendances_id_patch(id, attendance_update_request, x_personio_partner_id, x_personio_app_id)
Update Attendance by ID

This endpoint is responsible for updating attendance data for the company employees. Attributes are not required and if not specified, the current value will be used. It is not possible to change the employee id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the attendance period to update | [required] |
**attendance_update_request** | [**AttendanceUpdateRequest**](AttendanceUpdateRequest.md) | Attendance period data to update. At least one of the properties (`date`, `start_time`, `end_time`, `break`, `comment`, `project_id`, `skip_approval`) is required. | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::UpdatedAttendanceResponse**](UpdatedAttendanceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_post

> models::NewAttendancePeriodResponse company_attendances_post(attendance_create_request, x_personio_partner_id, x_personio_app_id)
Create an Attendance

This endpoint is responsible for adding attendance data for the company employees. It is possible to add attendances for one or many employees at the same time. The payload sent on the request should be a list of attendance periods, in the form of an array containing attendance period objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attendance_create_request** | [**AttendanceCreateRequest**](AttendanceCreateRequest.md) | List of attendance periods to create | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::NewAttendancePeriodResponse**](NewAttendancePeriodResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

