# Authorization Parsing for Rocket.rs 

A library for [Rocket](https://github.com/SergioBenitez/Rocket) web servers to easily access and parse `Authorization` headers from requests in the form of request guards. There is no functionality for performing authentication or generating valid login tokens.

The most common use case is for web micro-services where authentication has already happened elswhere, and the micro-service only needs to use and validate already generated authorization credentials.

Please look at the source within the [`example`](./example) directory for sample usage until proper documentation has been added.
