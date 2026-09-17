# Efficient Design Methods for Back-end API Documentation

### Essential Content
- `API Title` -> API name
- `URL` -> API path
- `Method` -> request method
- `Data Params` -> values to include in the body for POST requests
- `URL Params`(if applicable)
  - `Required` -> values to pass as URL params
- `Success Response` -> values and codes returned if the response is not successful
- `Sample Casll` -> request and response examples

The above content must be included, and API documentation can be written in a table format or using bullet points.

![api docs](./image/apidocs.png)

<br>

### How to Write Good API Documentation

#### 1. Write Documentation Before Starting Development

Consider writing the documentation before starting development. While it might seem counterintuitive to write documentation without having developed anything yet, simply outlining what the API will do and what prerequisites are needed can be very useful. This clarifies the direction of API development and allows you to share a draft with colleagues for feedback.

#### 2. State the Main Point Upfront

The beginning of the API documentation should broadly state what the API is intended to do. This helps readers determine if the document is relevant to their needs.
  
Furthermore, development API documentation will likely include a wide variety of features. It's good practice to explain basic information, such as authentication and header types required for API calls, in the introductory section, guiding first-time users on how to get started.

#### 3. Consider Your Audience

The audience for API documentation can broadly be divided into developers and users. You should write the documentation considering both developers and users.
  
Developers want to quickly navigate and test specific parts of the documentation. To facilitate this, it's necessary to provide a consistent document structure and examples. Specific considerations include using consistent headings, providing hyperlinks, and utilizing paragraph breaks and lists.
  
Users' skill levels can range from beginner to advanced. You should keep beginner users in mind while also ensuring advanced users can accurately find the information they need. In other words, it's best to include essential information accurately while using the simplest possible language. API documentation should be written clearly and concisely enough for someone without development knowledge to understand.

#### 4. Include Examples

Most developers prefer to copy and paste example code provided in API documentation. This allows them to conduct tests and quickly adapt the code as needed. It's good practice to provide examples that can test the API's core endpoints.
  
Additionally, you should include examples of successful responses and error responses. At this point, it's best to include only the most representative errors, not all of them, otherwise the documentation can become too long.

#### 5. Refer to Good API Documentation
Many companies already create and provide excellent API documentation. You can refer to examples cited as good API documentation by visiting the sites below.

- Stripe: https://stripe.com/docs
- Google: https://developers.google.com/gmail

#### 6. Allocate Sufficient Time

Writing good API documentation takes time. API documentation is not just a simple manual for developers and users; it's also part of the product itself. You should allocate sufficient time for API documentation.
  
If you're working in a team at a company, it's advisable to discuss the documentation thoroughly with other developers as well as technical writers. If someone within the team cannot understand the document, it means improvements are needed.

API documentation is not a one-time task. You must continuously update the documentation by incorporating feedback from developers and users.

<br>

### Summary
1. Write documentation before starting development
2. State the main point upfront
3. Consider your audience
4. Include examples
5. Refer to good API documentation
6. Allocate sufficient time
