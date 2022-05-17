# webserver-rs

WIP server of the webs that wants to become the framework of the webs.

---

## Design thoughts

Taking inspiration bits from Tide, Axum, and Dropshot.

- #### Web server and Web framework are not the same things?

Server: handles the work of implementing HTTP, getting a request, doing _something_, and returning a response.
Framework: handles the doing _something_ part.


```
Looks like this most of the time:
-----------------------
|Framework  --------  |
|           |Server|  |
|           |      |  |
|           |      |  |
|           |      |  |
|           --------  |
-----------------------

More flexible, loose coupling?
-----------------------------
| Interface                 |
|                           |
| -----------     --------  |
| |Framework|     |Server|  |
| |Core     |     |      |  |
| |         |     |      |  |
| |         |     |      |  |
| -----------     --------  |
-----------------------------

```

- #### What the heck is routing?

Routing is how the framework maps from an HTTP request to an endpoint, i.e. a piece of code intended to handle the request. [[Tide::routing]](https://rustasync.github.io/team/2018/10/16/tide-routing.html)


- #### Why is everything `async`? Where are the multi-`thread`ed web-frameworks?

Collective wisdom on the internet says that async make sense for I/O bound tasks and threads make sense for compute bound tasks. To quote [Some_Dev_Dood](https://www.reddit.com/r/rust/comments/ng3fws/comment/gyop3bh/?utm_source=share&utm_medium=web2x&context=3) on this,

> Generally speaking, we use async APIs when the application is IO-bound. That is, we are limited by the time it takes to receive input and send output. For instance, a TCP connection is IO-bound because most of the time, the CPU is just idle while waiting to receive the next few bytes. Instead of blocking the CPU, we use async APIs so that under the hood, a runtime (like tokio) can switch to a different task while waiting for the TCP connection. In turn, this helps us minimize the CPU's idle time.\
\
On the other hand, we use threading APIs when the application is CPU-bound. That is, we are limited by the CPU's processing power. Suppose we want to compute the digits of pi. Observe that we may only generate the digits as fast as the CPU can crunch the numbers—hence the term "CPU-bound". To address this, we may break our application into parallel components so that it's possible to use multiple threads to generate the digits.\
\
In a nutshell, async APIs are there to minimize the CPU idle time caused by IO. Meanwhile, threading APIs are there to maximize computation rate.

- #### Async?

You don't want your cpu to be idle. So you want it to pick up some other task. That is why you want a concurrency model to be there.

---