# \DocumentsApi

All URIs are relative to *https://api.personio.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**company_document_categories_get**](DocumentsApi.md#company_document_categories_get) | **GET** /company/document-categories | List Document Categories
[**company_documents_post**](DocumentsApi.md#company_documents_post) | **POST** /company/documents | Upload Document



## company_document_categories_get

> models::DocumentCategoriesResponse company_document_categories_get(x_personio_partner_id, x_personio_app_id)
List Document Categories

This endpoint is responsible for fetching all document categories of the company. The result contains a list of document categories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |

### Return type

[**models::DocumentCategoriesResponse**](DocumentCategoriesResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## company_documents_post

> models::DocumentUploadSuccessResponse company_documents_post(title, employee_id, category_id, file, x_personio_partner_id, x_personio_app_id, comment, date)
Upload Document

This endpoint is responsible for uploading documents for the company employees. The Documents endpoint has a rate limit of 60 requests per minute. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**title** | **String** | Title of the document. Maximum length is 255 characters. | [required] |
**employee_id** | **i32** | Employee identifier | [required] |
**category_id** | **i32** | Document Category identifier | [required] |
**file** | **std::path::PathBuf** | The document that shall be uploaded to an employees profile. Maximum file size is 30MB. | [required] |
**x_personio_partner_id** | Option<**String**> | The partner identifier |  |
**x_personio_app_id** | Option<**String**> | The application identifier |  |
**comment** | Option<**String**> | Optional comment that can be added to the uploaded document. |  |
**date** | Option<**String**> | Optional date can be added to the uploaded document. Must follow the format: Y-m-d |  |

### Return type

[**models::DocumentUploadSuccessResponse**](DocumentUploadSuccessResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

