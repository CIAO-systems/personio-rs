# \ProjectsApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_attendances_projects_get**](ProjectsApi.md#company_attendances_projects_get) | **GET** /company/attendances/projects | List Projects
[**company_attendances_projects_id_delete**](ProjectsApi.md#company_attendances_projects_id_delete) | **DELETE** /company/attendances/projects/{id} | Delete Project by ID
[**company_attendances_projects_id_patch**](ProjectsApi.md#company_attendances_projects_id_patch) | **PATCH** /company/attendances/projects/{id} | Update Project by ID
[**company_attendances_projects_post**](ProjectsApi.md#company_attendances_projects_post) | **POST** /company/attendances/projects | Create a Project



## company_attendances_projects_get

> models::CompanyAttendancesProjectsGet200Response company_attendances_projects_get(x_personio_partner_id, x_personio_app_id)
List Projects

Provides a list of all company projects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::CompanyAttendancesProjectsGet200Response**](_company_attendances_projects_get_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_projects_id_delete

> company_attendances_projects_id_delete(id)
Delete Project by ID

Deletes a project from the company account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Numeric `id` of the project | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_projects_id_patch

> models::CompanyAttendancesProjectsPost200Response company_attendances_projects_id_patch(id, company_attendances_projects_id_patch_request)
Update Project by ID

Updates a project with the given data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Numeric `id` of the project | [required] |
**company_attendances_projects_id_patch_request** | Option<[**CompanyAttendancesProjectsIdPatchRequest**](CompanyAttendancesProjectsIdPatchRequest.md)> |  |  |

### Return type

[**models::CompanyAttendancesProjectsPost200Response**](_company_attendances_projects_post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_attendances_projects_post

> models::CompanyAttendancesProjectsPost200Response company_attendances_projects_post(x_personio_partner_id, x_personio_app_id, company_attendances_projects_post_request)
Create a Project

Creates a project into the company account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**company_attendances_projects_post_request** | Option<[**CompanyAttendancesProjectsPostRequest**](CompanyAttendancesProjectsPostRequest.md)> |  |  |

### Return type

[**models::CompanyAttendancesProjectsPost200Response**](_company_attendances_projects_post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

