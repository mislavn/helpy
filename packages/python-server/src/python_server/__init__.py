import logging
import rust_server
from flask import Flask, jsonify, request
import my_logging


# Function that create the app
def create_app(test_config=None):
    # create and configure the app
    app = Flask(__name__)
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger(__name__)
    calculator = rust_server._core.RustClass()
    tracer = my_logging.create_tracer()

    # Python route
    @app.route("/python")
    def python_hello():
        with tracer.start_span("REST /python"):
            logger.info("Creating hello world message")
            return jsonify({"status": "success", "message": "Hello World!"})

    # Rust route
    @app.route("/rust")
    def rust_hello():
        with tracer.start_span("REST /rust"):
            logger.info("Creating hello world message")
            return rust_server.hello()

    # Rust error route
    @app.route("/rust/error")
    def rust_error():
        with tracer.start_span("REST /rust/error"):
            logger.info("Rust error")
            return rust_server.error()

    # Rust error route
    @app.route("/rust/calculator/counter")
    def calculator_counter():
        with tracer.start_span("REST /rust/calculator/counter") as span:
            logger.info("Calculator counter")
            return jsonify(
                calculator.counter(span.context.trace_id, span.context.span_id)
            )

    # Rust error route
    @app.route("/rust/calculator/sum_array", methods=["POST"])
    def calculator_sum():
        with tracer.start_span("REST /rust/calculator/sum_array") as span:
            logger.info("Log Calculator sum")
            input = request.json
            span.set_tag("input", input)
            span.log_event("Span Calculator sum")
            return jsonify(
                calculator.sum_array(input, span.context.trace_id, span.context.span_id)
            )

    return app  # do not forget to return the app


APP = create_app()


def main() -> None:
    APP.run(host="0.0.0.0", port=8000, debug=False)
