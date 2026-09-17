# Events, Handlers, Dispatchers

## Event Overview
> Events can remove dependencies between domains and facilitate post-processing.

The term "event" refers to **something that happened in the past**.

An event occurring means that a state has changed.

We implement functionality that doesn't just end with an event occurring, but reacts to that event to perform desired actions.

Requirements such as "when...", "if... occurs", or "if..." often relate to changes in a domain's state and can be implemented using events.

For example, the requirement to send an email when an order is canceled means that the order's state changes to "canceled," and this can be implemented using an 'Order Canceled Event'.

## Event-Related Components
To introduce events into a domain model, the following four components are needed:
- Event
- Creator
- Dispatcher (Publisher)
- Handler (Subscriber)

![](https://user-images.githubusercontent.com/42582516/160224240-ca76a9a2-321c-456b-a27b-d745d5b36b0e.png)
Event-related components

### Event Creator
- The event creator is a domain object such as an entity, value, or domain service.
- When domain logic is executed and the state changes, it raises a related event.

### Event Handler
- Reacts to the occurred event.
- Uses the data contained in the event to execute desired functionality.

### Event Dispatcher
- Connects the event creator and the event handler.
- Depending on the implementation method of the event dispatcher, event creation and processing can be executed synchronously or asynchronously.

## Event Structure
An event contains the following information:
- Event type - expressed by the class name
- Event occurrence time
- Additional data - information related to the event, such as order number, new shipping information

As shown below, an event is a class that uses the past tense.

```java
public class ShippingInfoChangedEvent {
  
  private String orderNumber;
  private long timestamp;
  private ShippingInfo newShippingInfo;

  // 생성자, getter
}
```

The aggregate that becomes the creator of the event is as follows.
`Events.raise()` provides the functionality to propagate events through a dispatcher.

```java
public class Order {
  public void changeShippingInfo(ShippingInfo newShippingInfo) {
    verifyNotYetShipped();
    setShippingInfo(newShippingInfo);
    Events.raise(new ShippingInfoChangedEvent(number, newShippingInfo));
  }
  ...
}
```
A handler that performs necessary tasks from the dispatcher can be implemented as follows.

```java
public class ShippingInfoChangedHandler {

  @EventListener(ShippingInfoChangedEvent.class)
  public void handle(ShppingInfoChangedEvent evt) {
    shippingInfoSynchronizer.sync(
      evt.getOrderNumber(),
      evt,getNewShippingInfo());
  }
  ...
}
```

An event must contain the data necessary for the event handler to perform its task.
While an event must contain data, it does not need to include data unrelated to the event itself.

## Event Use Cases
Events are primarily used for two purposes:
- Trigger
- Data synchronization

### Trigger
When post-processing is required after a domain's state changes, events can be used as a trigger to execute that post-processing.
For example, notifying booking results via SMS or processing a refund when an order is canceled can be implemented using events.
If additional functionality is added, it can be resolved by creating a handler to process that functionality.

![](https://user-images.githubusercontent.com/43809168/99668375-fe0fef80-2ab0-11eb-8e78-7bcbd78e84c8.png)
Event Trigger

## Implementing Events, Handlers, and Dispatchers
> Easily implemented using Spring's ApplicationEventPublisher.

- Event class: Represents an event.
- Dispatcher: Uses Spring's `ApplicationEventPublisher`.
- Events: Publishes events. Uses `ApplicationEventPublisher` for event publishing.
- Event Handler: Receives and processes events. Uses functionality provided by Spring.

## Event Class
Since an event signifies a state change or incident that occurred in the past, the class name should use the past tense.
If all events share common properties, a related superclass can be created.

## Events Class and ApplicationEventPublisher
Spring's `ApplicationEventPublisher` is used for event occurrence and publishing.
The Spring container also acts as an `ApplicationEventPublisher`.

```java
public class Events {
  private static ApplicationEventPublisher publisher;
  
  static void setPublisher(ApplicationEventPublisher publisher) {
    Events.publisher = publisher;
  }
  
  public static void raise(Object event) {
    if (publisher != null) {
      publisher.publishEvent(event);
    }
  }
}
```

The `Events` class uses `ApplicationEventPublisher` to raise events.
To pass the event publisher to the `setPublisher()` method of the `Events` class, a Spring configuration class is written.

```java
@Configuration
public class EventConfiguration {
  private ApplicationContext applicationContext;
  
  @Bean
  public InitializingBean eventsInitializer() {
    return () -> Events.setPublisher(applicationContext);
  }
}
```

`InitializingBean` is an interface used to initialize Spring bean objects. It passes `ApplicationContext`, which inherits `ApplicationEventPublisher`, to initialize the `Events` class.

## Event Occurrence and Event Handler
Perform the relevant domain logic and use `Events.raise()` to raise the associated event.
To handle the raised event, implement the handler as follows.

```java
@Service
public class OrderCanceledEventHandler() {
  private RefundService refundService;
  
  @EventListener(OrderCanceledEvent.class)
  public void handle(OrderCanceledEvent event) {
    refundService.refund(event.getOrderNumber());
  }
}
```

## Flow Summary
The flow of event processing:
1. Execute domain functionality.
2. The domain functionality raises an event using `Events.raise()`.
3. `Events.raise()` publishes the event using Spring's `ApplicationEventPublisher`.
4. `ApplicationEventPublisher` finds and executes methods annotated with `@EventListener(EventType.class)`.

Domain state changes and event handlers are executed within the same transaction scope.
