import logging
from jaeger_client import Config


def create_tracer(name="PyServer"):
    log_level = logging.INFO
    logging.getLogger("").handlers = []
    logging.basicConfig(format="%(asctime)s %(message)s", level=log_level)

    config = Config(
        config={  # usually read from some yaml config
            "sampler": {
                "type": "const",
                "param": 1,
            },
            "local_agent": {
                # Specify the hostname and port number of the Jaeger agent.
                # To ensure data reliability, we recommend that you run the Jaeger client and the Jaeger agent on the same host. Therefore, the reporting_host parameter is set to 127.0.0.1.
                "reporting_host": "127.0.0.1",
                "reporting_port": 6831,
            },
            "logging": True,
        },
        # Specify the application name.
        service_name=name,
        validate=True,
    )

    # this call also sets opentracing.tracer
    return config.initialize_tracer()
