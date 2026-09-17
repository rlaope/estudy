# Three QoS Implementation Models (Best Effort, IntServ, DiffServ)

### QoS (Quality of Service)

When a network only transmits data, speed is not particularly important.

However, in interactive applications that deliver audio and video content, speed is sensitive.

If packets are lost during a video conference, causing stream delays, that would obviously be unacceptable.

Therefore, there are models for QoS processing, so let's look at them one by one.

<br>

### IntServ (Integrated Services)

This model uses Resource Reserve Protocol (RSVP) to reserve network bandwidth resources and request QoS processing.

Its purpose is for applications to signal the network to guarantee bandwidth expected to be needed for end-to-end communication, but it is difficult to implement in reality and is not preferred due to scalability issues.

To use end-to-end QoS on all nodes, RSVP path states must be maintained for all traffic, and maintaining path states ultimately means difficulties with scalability.

This is its biggest drawback and why IntServ cannot be used in large-scale networks.

<br>

### DiffServ (Differentiated Services)

It was born to overcome the limitations of the BestEffort and IntServ models, and it **identifies and classifies incoming network traffic to perform QoS. IP traffic is classified into multiple classes according to business requirements, and a different service level is assigned to each class, applying QoS.** The processing order is determined by the level of the classified service.

The DiffServ model can classify services and process them according to priority, and lower-priority services can be discarded to avoid congestion, making it the most widely used model for QoS application. In the DiffServ model, DSCP is applied to classify services and apply QoS policies. Since QoS policies are executed independently for each network device, policies must be configured on all network devices for traffic flows that require QoS application.

<br>

In conclusion, IntServ's resource reservation method using RSVP requires maintaining state information for routers for each flow, and this mode of operation is complex to manage. Therefore, DiffServ, which marks the DSCP field and processes each packet differently, is much more advantageous in terms of efficiency and scalability.

DSCP uses 6 bits, so it can be represented by 2^6, or 62 different binaries, but commonly used binaries, DSCP names, and IP priorities can be checked.

Let's look at the 4 types of DiffServ Architecture - PHB (Packet Handling Behavior specified by routers):
1. **BE (Best Effort)**: As seen above, general packet processing with no priority, simply striving to deliver packets quickly.
2. **EF (Expedited Forwarding)**: Processes packets with the highest priority, allocating minimal latency and maximum bandwidth for high-priority packets, ensuring the transmission of high-priority packets.
3. **AF (Assured Forwarding)**: Applies bandwidth guarantees for service classes or categories, allocating bandwidth per service class or category, and discarding specific packets if they exceed their allocated bandwidth.
4. **CS (Class Selector)**: Uses existing IP service classes, checking the DSCP value corresponding to the existing IP service class to apply the appropriate PHB.
