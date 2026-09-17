# SecurityContextHolderStrategy (ThreadLocal, InheritableThreadLocal, Global)

## TheradLocal
Assuming a project is built on WebMVC, one thread is created per request.

In this scenario, using ThreadLocal allows creating a unique space for each thread, where the SecurityContext can be stored.

- If SecurityContext is managed with ThreadLocal, the SecurityContext is managed independently for each request.
- Uses `IngeritableThreadSecurityLocalSecurityContextHolderStrategy`.

## InheritableThreadLocal

Uses `InheritableThreadSecurityLocalContextHolderStrategy`, which allows sharing the SecurityContext even with child threads.

## Global

Uses `GlobalSecurityContextHolderStrategy`, which is set globally and shares the SecurityContext across the entire application.
