package learn.project9.demo

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.cloud.gateway.route.RouteLocator
import org.springframework.cloud.gateway.route.builder.RouteLocatorBuilder
import org.springframework.cloud.gateway.route.builder.filters
import org.springframework.cloud.gateway.route.builder.routes
import org.springframework.context.annotation.Bean

@SpringBootApplication
class DemoApplication {

    @Bean
    fun customRouteLocator(builder: RouteLocatorBuilder): RouteLocator {
        return builder.routes {
            route("root") {
                path("/")
                filters {
                    addResponseHeader("X-Custom-Header", "RootRouteHeader")
                }
                uri("localhost:8081")
            }
            route("kek") {
                path("/kek")
                filters {
                    addResponseHeader("X-Custom-Header", "KekRouterHeader")
                    addResponseHeader("X-Debug-Info", "Gateway successfully routed request to localhost:8081")
                }
                uri("localhost:8081/kek")
            }
            route("testing") {
                path("/get")
                filters {
                    addRequestHeader("Hello", "World")
                }
                uri("http://httpbin.org:80")
            }
        }

    }
}

fun main(args: Array<String>) {
    runApplication<DemoApplication>(*args)
}
