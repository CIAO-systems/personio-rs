# \CustomReportsApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_columns**](CustomReportsApi.md#list_columns) | **GET** /company/custom-reports/columns | List Custom Report Column Labels
[**list_report_items**](CustomReportsApi.md#list_report_items) | **GET** /company/custom-reports/reports/{report_id} | Get Custom Report by ID
[**list_reports**](CustomReportsApi.md#list_reports) | **GET** /company/custom-reports/reports | List Custom Reports



## list_columns

> models::PublicListColumnsResponse list_columns(x_personio_partner_id, x_personio_app_id, columns, locale, report_id)
List Custom Report Column Labels

This endpoint provides human-readable labels for report table columns. It is particularly important if you get a report with custom attributes or absence data to match the column IDs to the translation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**columns** | Option<[**Vec<String>**](String.md)> | The columns to filter the results. |  |
**locale** | Option<**String**> | locale used to translate localized fields. |  |
**report_id** | Option<**String**> | The ID of the report to filter the result of the columns. If no ID is passed, all columns for the company are returned. |  |

### Return type

[**models::PublicListColumnsResponse**](PublicListColumnsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_report_items

> models::PublicReportResponse list_report_items(report_id, x_personio_partner_id, x_personio_app_id, locale, page, limit)
Get Custom Report by ID

This endpoint provides you with the data of an existing Custom Report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | The ID of the report to filter the result. | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**locale** | Option<**String**> | locale used to translate localized fields. |  |
**page** | Option<**i32**> | Pagination parameter to identify the page to return. |  |
**limit** | Option<**i32**> | Pagination parameter to limit the number of employees returned per page. |  |

### Return type

[**models::PublicReportResponse**](PublicReportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_reports

> models::PublicListReportsResponse list_reports(x_personio_partner_id, x_personio_app_id, report_ids, status)
List Custom Reports

This endpoint provides you with metadata about existing custom reports in your Personio account, such as report name, report type, report date / timeframe.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**report_ids** | Option<[**Vec<String>**](String.md)> | A list of report ID's to filter the results. |  |
**status** | Option<**String**> | The status of the report to filter the results. |  |

### Return type

[**models::PublicListReportsResponse**](PublicListReportsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

