package learn.project9.demo

import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.call
import io.ktor.server.engine.embeddedServer
import io.ktor.server.http.content.static
import io.ktor.server.http.content.staticFiles
import io.ktor.server.http.content.staticResources
import io.ktor.server.netty.Netty
import io.ktor.server.response.header
import io.ktor.server.response.respondText
import io.ktor.server.routing.get
import io.ktor.server.routing.routing
import io.ktor.server.util.getOrFail
import kotlinx.coroutines.delay
import java.io.File

fun main() {
    embeddedServer(Netty, port = 8081, host = "0.0.0.0", module = Application::module)
        .start(wait = true)
}

fun Application.configureRoutes() {
    routing {
        get("/") {
            call.respondText("Hello World!")
        }
        get("/header") {
            call.response.header("X-TG-ClientId", "client-id")
            call.response.header("X-TG-UserId", "user-id")
            call.response.header("X-TG-new-custom-header", "new-custom-header")
            call.response.header(HttpHeaders.ContentType, "application/json")
            call.respondText(
                """
                {
                   "key": "value"
                }
            """.trimIndent()
            )
        }
        // /status&status=XXX where XXX = number
        get("/status") {
            val status = call.request.queryParameters.getOrFail("status")
            call.response.status(HttpStatusCode.fromValue(status.toInt()))
        }

        get("/slow") {
            delay(1_500)
            call.respondText { """
                {
                    "success": "yes"
                }
            """.trimIndent() }
        }
    }

}

fun Application.module() {
    configureRoutes()
}
