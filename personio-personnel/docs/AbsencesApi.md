# \AbsencesApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_absence_periods_get**](AbsencesApi.md#company_absence_periods_get) | **GET** /company/absence-periods | List Absence Periods
[**company_absence_periods_id_delete**](AbsencesApi.md#company_absence_periods_id_delete) | **DELETE** /company/absence-periods/{id} | Delete Absence Period by ID
[**company_absence_periods_post**](AbsencesApi.md#company_absence_periods_post) | **POST** /company/absence-periods | Create an Absence Period
[**company_time_off_types_get**](AbsencesApi.md#company_time_off_types_get) | **GET** /company/time-off-types | List Time-Off Types
[**company_time_offs_get**](AbsencesApi.md#company_time_offs_get) | **GET** /company/time-offs | List Time-Offs
[**company_time_offs_id_delete**](AbsencesApi.md#company_time_offs_id_delete) | **DELETE** /company/time-offs/{id} | Delete Time-Off by ID
[**company_time_offs_id_get**](AbsencesApi.md#company_time_offs_id_get) | **GET** /company/time-offs/{id} | Get Time-Off by ID
[**company_time_offs_post**](AbsencesApi.md#company_time_offs_post) | **POST** /company/time-offs | Create a Time-Off



## company_absence_periods_get

> models::HourlyAbsencePeriodsResponse company_absence_periods_get(x_personio_partner_id, x_personio_app_id, start_date, end_date, updated_from, updated_to, employees_left_square_bracket_right_square_bracket, absence_types_left_square_bracket_right_square_bracket, absence_periods_left_square_bracket_right_square_bracket, limit, offset)
List Absence Periods

Fetches absence periods for absences with **time unit** set to **hours**. The result can be `paginated` and `filtered` by period and/or specific employee/employees. The result contains a list of hourly absence periods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**start_date** | Option<**String**> | When both start_date and end_date values are provided, the list of absences returned is a set that has all absences that intersects the window expressed as [start_date, end_date]. When either value is unspecified, the list of absences returned are unbounded in the direction of the unspecified value. |  |
**end_date** | Option<**String**> | When both start_date and end_date values are provided, the list of absences returned is a set that has all absences that intersects the window expressed as [start_date, end_date]. When either value is unspecified, the list of absences returned are unbounded in the direction of the unspecified value. |  |
**updated_from** | Option<**String**> | Filter by periods that were created or modified from the date updated_from. It is inclusive, so all the periods created or modified from the beginning of the updated_from will be included in the results. |  |
**updated_to** | Option<**String**> | Filter by periods that were created or modified until the date updated_to. It is inclusive, so all the periods created or modified until the end of the updated_to will be included in the results. |  |
**employees_left_square_bracket_right_square_bracket** | Option<[**Vec<i32>**](i32.md)> | A list of Personio employee ID's to filter the results. The result filters including only absences of provided employees |  |
**absence_types_left_square_bracket_right_square_bracket** | Option<[**Vec<String>**](String.md)> | A list of Personio absence type IDs to filter the results. The result filters including only absences of provided absence types. |  |
**absence_periods_left_square_bracket_right_square_bracket** | Option<[**Vec<String>**](String.md)> | A list of Personio absence period IDs to filter the results. The result filters including only absences containing the provided ids. |  |
**limit** | Option<**i32**> | Pagination attribute to limit how many absence periods are returned per page. |  |[default to 200]
**offset** | Option<**i32**> | Pagination attribute to identify which page you are requesting, by the form of telling an offset from the first record that would be returned. |  |[default to 0]

### Return type

[**models::HourlyAbsencePeriodsResponse**](HourlyAbsencePeriodsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_absence_periods_id_delete

> models::DeletedAbsenceResponse company_absence_periods_id_delete(id, x_personio_partner_id, x_personio_app_id)
Delete Absence Period by ID

Deletes absence period data for absence types with **time unit** set to **hours**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the absence period to delete | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::DeletedAbsenceResponse**](DeletedAbsenceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_absence_periods_post

> models::CompanyAbsencePeriodsPost201Response company_absence_periods_post(employee_id, time_off_type_id, start_date, end_date, x_personio_partner_id, x_personio_app_id, start_time, end_time, half_day_start, half_day_end, comment, skip_approval)
Create an Absence Period

Adds absence data for absence types with **time unit** set to **hours**. Note that creating periods for absence types with certificate requirement enabled is not supported!

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Employee identifier | [required] |
**time_off_type_id** | **i32** | Time-off type identifier | [required] |
**start_date** | **String** | Absence start date. Format: yyyy-mm-dd | [required] |
**end_date** | **String** | Absence end date. Format: yyyy-mm-dd | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**start_time** | Option<**String**> | Absence start time. Format: HH:mm (hours and minutes) and only required if the start_date and end_date are the same (partial-day absence) |  |
**end_time** | Option<**String**> | Absence end time. Format: HH:mm (hours and minutes) and only required if the start_date and end_date are the same (partial-day absence) |  |
**half_day_start** | Option<**bool**> | Whether the start date is a half-day off, only considered if the start_date and end_date are not the same (more than one day absence) |  |
**half_day_end** | Option<**bool**> | Whether the end date is a half-day off, only considered if the start_date and end_date are not the same (more than one day absence) |  |
**comment** | Option<**String**> | Optional comment |  |
**skip_approval** | Option<**bool**> | Optional, default value is true. If set to false, the approval status of the absence request will be \\\"pending\\\" if an approval rule is set for the absence type in Personio. The respective approval flow will be triggered. |  |

### Return type

[**models::CompanyAbsencePeriodsPost201Response**](_company_absence_periods_post_201_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_time_off_types_get

> models::CompanyTimeOffTypesGet200Response company_time_off_types_get(x_personio_partner_id, x_personio_app_id, limit, offset)
List Time-Off Types

Provides a list of absence types for absences **time unit** set to either **days** or **hours**. For example 'Paid vacation', 'Parental leave' or 'Home office'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**limit** | Option<**i32**> | Pagination attribute to limit how many records will be returned per page |  |[default to 200]
**offset** | Option<**i32**> | Pagination attribute to identify which page you are requesting, by the form of telling an offset from the first record that would be returned. |  |[default to 0]

### Return type

[**models::CompanyTimeOffTypesGet200Response**](_company_time_off_types_get_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_time_offs_get

> models::AbsencePeriodsResponse company_time_offs_get(x_personio_partner_id, x_personio_app_id, start_date, end_date, updated_from, updated_to, employees_left_square_bracket_right_square_bracket, limit, offset)
List Time-Offs

Fetches absence periods for absences with **time unit** set to **days**. The result can be `paginated` and `filtered` by period and/or specific employee/employees. The result contains a list of absence periods.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**start_date** | Option<**String**> | First day of the period to be queried. It is inclusive, so the result starts from and including the provided `start_date` |  |
**end_date** | Option<**String**> | Last day of the period to be queried. It is inclusive, so the result ends on `end_date` including absences from the `end_date` |  |
**updated_from** | Option<**String**> | Query the periods that created or modified from the date `updated_from`. It is inclusive, so all the periods created or modified from the beginning of the `updated_from` will be included in the results |  |
**updated_to** | Option<**String**> | Query the periods that created or modified until the date `updated_to`. It is inclusive, so all the periods created or modified until the end of the `updated_to` will be included in the results |  |
**employees_left_square_bracket_right_square_bracket** | Option<[**Vec<i32>**](i32.md)> | A list of Personio employee ID's to filter the results. The result filters including only absences of provided employees |  |
**limit** | Option<**i32**> | Pagination attribute to limit the number of absence periods per page |  |[default to 200]
**offset** | Option<**i32**> | Pagination attribute to identify which page number you are requesting |  |[default to 0]

### Return type

[**models::AbsencePeriodsResponse**](AbsencePeriodsResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_time_offs_id_delete

> models::DeletedAbsenceResponse company_time_offs_id_delete(id, x_personio_partner_id, x_personio_app_id)
Delete Time-Off by ID

Deletes absence period data for absence types with **time unit** set to **days**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | ID of the absence period to delete | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::DeletedAbsenceResponse**](DeletedAbsenceResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_time_offs_id_get

> models::AbsencePeriodResponse company_time_offs_id_get(id, x_personio_partner_id, x_personio_app_id)
Get Time-Off by ID

Gets an absence period for absences with **time unit** set to **days**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Numeric `id` of the absence period | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::AbsencePeriodResponse**](AbsencePeriodResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_time_offs_post

> models::CompanyTimeOffsPost200Response company_time_offs_post(employee_id, time_off_type_id, start_date, end_date, half_day_start, half_day_end, x_personio_partner_id, x_personio_app_id, comment, skip_approval)
Create a Time-Off

Adds absence data for absence types with **time unit** set to **days**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**employee_id** | **i32** | Employee identifier | [required] |
**time_off_type_id** | **i32** | Time-off type identifier | [required] |
**start_date** | **String** | Absence start date. Format: yyyy-mm-dd | [required] |
**end_date** | **String** | Absence end date. Format: yyyy-mm-dd | [required] |
**half_day_start** | **bool** | Whether the start date is a half-day off. | [required] |
**half_day_end** | **bool** | Whether the end date is a half-day off. | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**comment** | Option<**String**> | Optional comment |  |
**skip_approval** | Option<**bool**> | Optional, default value is true. If set to false, the approval status of the absence request will be \\\"pending\\\" if an approval rule is set for the absence type in Personio. The respective approval flow will be triggered. |  |

### Return type

[**models::CompanyTimeOffsPost200Response**](_company_time_offs_post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

