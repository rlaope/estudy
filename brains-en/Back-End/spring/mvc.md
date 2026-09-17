# What is the MVC Pattern?

### MVC Pattern
It's an abbreviation for `Model-View-Controller`, a development methodology that divides roles into three distinct parts during development.

![mvc](./image/mvcp.png)

- `Model`: This part defines what the application will do. In other words, it interacts with the database to handle data input by the user or data to be output to the user.
- `View`: This part is what is visually shown to the user. (UI)
- `Controller`: This role tells the Model how to process data. If there is data sent by the client, it processes it appropriately before calling the Model. Then, once the Model completes its task, it takes the results and passes them to the View.

<br>

### Why Use the MVC Pattern?
- If you create an application consisting of these three parts—the page the user sees, data processing, and the controller that mediates between these two—each part can focus solely on its assigned responsibility.
- In other words, by developing an application where parts are separated and can focus on their individual roles, `maintainability`, `application scalability`, and `flexibility` increase, and the problem of duplicate code also disappears.
