package learn.project9.demo

import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.engine.embeddedServer
import io.ktor.server.netty.Netty
import io.ktor.server.response.header
import io.ktor.server.response.respondText
import io.ktor.server.routing.get
import io.ktor.server.routing.routing

fun main() {
    embeddedServer(Netty, port = 8081, host = "0.0.0.0", module = Application::module)
        .start(wait = true)
}

fun Application.configureRoutes() {
    routing {
        get("/") {
            println("a call to /")
            call.respondText("Hello World!")
        }
        get("/header") {
            println("a call to /header")
            call.response.header("X-TG-ClientId", "value")
            call.respondText(
                """
                {
                   "key": "value"
                }
            """.trimIndent()
            )
        }
    }

}

fun Application.module() {
    configureRoutes()
}
