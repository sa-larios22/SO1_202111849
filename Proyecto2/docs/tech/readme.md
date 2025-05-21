# Manual Técnico

## Introducción
El presente proyecto tiene como finalidad integrar conocimientos fundamentales sobre sistemas operativos, programación concurrente, contenedores, mensajería distribuida y arquitectura en la nube, desarrollados durante las primeras unidades del curso. La propuesta se centra en la implementación de un sistema distribuido de alta concurrencia desplegado en Google Kubernetes Engine (GKE), capaz de recibir, procesar y almacenar miles de tuits simulados relacionados con el clima mundial.

Para ello, se hace uso de una arquitectura modular basada en microservicios containerizados, desarrollados principalmente en los lenguajes Rust y Go, los cuales aprovechan las capacidades de concurrencia de estos lenguajes. Los mensajes son distribuidos usando los message brokers Kafka y RabbitMQ, y posteriormente consumidos y almacenados en las bases de datos en memoria Redis y Valkey, respectivamente. La visualización final de los datos se realiza por medio de Grafana, la cual permite monitorear de forma gráfica la cantidad y tipo de mensajes procesados.

El flujo de mensajes es generado por Locust, una herramienta que simula tráfico concurrente de usuarios enviando tuits con descripciones climáticas. Esto permite poner a prueba la escalabilidad del sistema mediante técnicas como el Horizontal Pod Autoscaler (HPA).

Finalmente, todas las imágenes Docker generadas se almacenan en un registro privado utilizando Harbor, alojado en una instancia de máquina virtual en Google Cloud, y todo el proyecto es versionado en un repositorio privado de GitHub.

## Objetivos
### Objetivo General
Diseñar e implementar un sistema distribuido y concurrente para la recepción, procesamiento, almacenamiento y visualización de tuits meteorológicos, utilizando tecnologías modernas como Kubernetes, microservicios en Rust y Go, RabbitMQ, Kafka, Redis, Valkey y Grafana, garantizando escalabilidad, eficiencia y despliegue en la nube mediante GKE.

### Objetivos Específicos
1. Desarrollar microservicios que simulen la recepción de tuits meteorológicos y los distribuyan a través de protocolos HTTP y gRPC.

2. Implementar brokers de mensajería (Kafka y RabbitMQ) para la distribución eficiente y desacoplada de mensajes entre productores y consumidores.

3. Consumir, procesar y almacenar los mensajes en sistemas de almacenamiento en memoria como Redis y Valkey para mejorar el rendimiento y la velocidad de acceso a los datos.

4. Visualizar la información procesada mediante paneles de Grafana que permitan monitorear la actividad del sistema en tiempo real.

5. Automatizar el despliegue en Kubernetes (GKE) utilizando Helm y manifiestos YAML, integrando mecanismos de escalado automático (HPA) y pruebas de carga concurrente con Locust.

## Tecnologías Utilizadas
- **Rust**: Lenguaje de programación enfocado en seguridad y alto rendimiento, utilizado para implementar el servicio principal de recepción de tuits.
- **Go (Golang)**: Lenguaje concurrente utilizado para desarrollar el cliente gRPC y los consumidores de mensajes.
- **Kubernetes (GKE)**: Plataforma de orquestación de contenedores usada para desplegar y escalar los microservicios en Google Cloud.
- **Docker**: Herramienta de contenedorización empleada para encapsular cada componente del sistema.
- **Kafka**: Sistema de mensajería distribuido usado para el envío y consumo eficiente de tuits procesados.
- **RabbitMQ**: Broker de mensajería usado como alternativa a Kafka para comparar flujos de procesamiento.
- **Redis**: Base de datos en memoria utilizada para almacenar mensajes provenientes de RabbitMQ.
- **Valkey**: Fork comunitario de Redis, usado para almacenar mensajes provenientes de Kafka.
- **Grafana**: Plataforma de visualización que permite monitorear gráficamente los datos almacenados.
- **Helm**: Gestor de paquetes para Kubernetes usado para automatizar despliegues.
- **Locust**: Herramienta de carga que simula usuarios concurrentes enviando tuits al sistema.
- **Harbor**: Registro privado de imágenes Docker autoalojado para almacenar los contenedores del proyecto.
- **GitHub**: Plataforma utilizada para el control de versiones y documentación del proyecto.

## Deployments
### Namespace

El archivo `Namespace.yaml` crea un nuevo espacio de nombres en Kubernetes llamado `weather-tweets`. Un **namespace** en Kubernetes actúa como una división lógica dentro del clúster, permitiendo agrupar y aislar recursos como pods, servicios, deployments, secretos y configuraciones. Esto es especialmente útil en entornos donde se manejan múltiples proyectos, entornos (como desarrollo, pruebas y producción) o equipos de trabajo.

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: weather-tweets
  labels:
    name: weather-tweets
```

### Ingress

El archivo `Ingress.yaml` define un recurso de tipo `Ingress` en el namespace `weather-tweets`, que permite enrutar peticiones HTTP externas hacia servicios internos desplegados en el clúster de Kubernetes. Este Ingress está configurado para ser gestionado por el controlador **NGINX**, como lo indica la anotación `kubernetes.io/ingress.class: "nginx"`. Además, se desactiva la redirección automática de HTTP a HTTPS mediante la directiva `nginx.ingress.kubernetes.io/ssl-redirect: "false"`, lo cual es útil en entornos de pruebas donde aún no se dispone de certificados SSL. También se habilita el uso de expresiones regulares en las rutas con la anotación `nginx.ingress.kubernetes.io/use-regex: "true"`.

La especificación de reglas (`spec.rules`) define que todas las solicitudes dirigidas al dominio `34.41.116.132.nip.io` serán procesadas por este Ingress. Este dominio se genera automáticamente utilizando el servicio gratuito [nip.io](https://nip.io), que resuelve subdominios basados en direcciones IP públicas, facilitando así el acceso sin necesidad de configurar un DNS personalizado. Dentro de las reglas, se especifica que cualquier solicitud HTTP con un prefijo de ruta `/input` será redirigida al servicio interno `rust-api-service`, expuesto en el puerto `8080`. Esto significa que cualquier cliente externo podrá acceder a la funcionalidad del microservicio desarrollado en Rust simplemente haciendo una petición HTTP al endpoint `http://34.41.116.132.nip.io/input`.

Este Ingress facilita la exposición controlada de servicios dentro del clúster a través de una interfaz pública, y forma parte clave de la arquitectura distribuida del proyecto, al actuar como puerta de entrada al sistema desde el exterior.

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: weather-tweets-ingress
  namespace: weather-tweets
  annotations:
    kubernetes.io/ingress.class: "nginx"
    nginx.ingress.kubernetes.io/ssl-redirect: "false"
    nginx.ingress.kubernetes.io/use-regex: "true"
spec:
  rules:
  - host: "34.41.116.132.nip.io"
    http:
      paths:
      - path: /input
        pathType: Prefix
        backend:
          service:
            name: rust-api-service
            port:
              number: 8080
```

### Go
#### go-client-deployment.yaml

El archivo `go-client-deployment.yaml` define un Deployment de Kubernetes que lanza dos réplicas del microservicio `go-client` dentro del namespace `weather-tweets`. Este servicio está basado en la imagen Docker `sergiolarios/go-client:latest`, expone el puerto `8081` y se le asignan límites y solicitudes moderados de CPU y memoria para optimizar su ejecución en entornos controlados. Además, se configura una variable de entorno `KAFKA_SERVER` que apunta al broker de Kafka `my-cluster-kafka-bootstrap:9092`, permitiendo que el cliente Go se conecte directamente al sistema de mensajería para reenviar los tuits recibidos desde el servicio en Rust.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: go-client
  namespace: weather-tweets
spec:
  replicas: 2
  selector:
    matchLabels:
      app: go-client
  template:
    metadata:
      labels:
        app: go-client
    spec:
      containers:
      - name: go-client
        image: sergiolarios/go-client:latest
        ports:
        - containerPort: 8081
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "500m"
            memory: "256Mi"
        env:
          - name: KAFKA_SERVER
            value: "my-cluster-kafka-bootstrap:9092"
```

#### go-client-service.yaml

El archivo `go-client-service.yaml` crea un recurso `Service` de tipo `ClusterIP` dentro del namespace `weather-tweets`, el cual expone internamente el microservicio que tiene la etiqueta `app: go-client` en el puerto `8081`. Este servicio permite que otros pods dentro del mismo clúster puedan comunicarse con el cliente Go a través del nombre `go-client-service`, enrutando las peticiones al puerto correspondiente del contenedor sin exponerlo al exterior del clúster.


```yaml
apiVersion: v1
kind: Service
metadata:
  name: go-client-service
  namespace: weather-tweets
spec:
  selector:
    app: go-client
  ports:
  - port: 8081
    targetPort: 8081
  type: ClusterIP
```

### Kafka
#### kafka-cluster.yaml

Este manifiesto define una instancia de Kafka desplegada mediante Strimzi dentro del namespace `weather-tweets`, con el nombre `my-cluster`. Se configura un solo broker Kafka (`replicas: 1`) con un listener interno sin TLS expuesto en el puerto `9092`, ideal para comunicaciones internas entre microservicios. La configuración de replicación está ajustada al mínimo para ambientes de desarrollo o prueba. Además, se incluye un nodo de Zookeeper también con almacenamiento efímero, necesario para la operación de Kafka, y se habilitan los operadores de usuario y de tópicos (`entityOperator`) para facilitar la gestión automática de usuarios y temas dentro del clúster.

```yaml
apiVersion: kafka.strimzi.io/v1beta2
kind: Kafka
metadata:
  name: my-cluster
  namespace: weather-tweets
spec:
  kafka:
    version: 4.0.0
    replicas: 1
    listeners:
      - name: plain
        port: 9092
        type: internal
        tls: false
    config:
      offsets.topic.replication.factor: 1
      transaction.state.log.replication.factor: 1
      transaction.state.log.min.isr: 1
    storage:
      type: ephemeral
  zookeeper:
    replicas: 1
    storage:
      type: ephemeral
  entityOperator:
    topicOperator: {}
    userOperator: {}
```

#### kafka-consumer-deployment.yaml

El archivo `kafka-consumer-deployment.yaml` define un Deployment que lanza una réplica del contenedor `sergiolarios/kafka-consumer:latest` en el namespace `weather-tweets`, encargado de consumir mensajes desde Kafka y almacenarlos en Redis. El contenedor utiliza dos variables de entorno: `REDIS_HOST`, que apunta al servicio interno de Redis (`redis-service:6379`), y `REDIS_PASSWORD`, que se obtiene de un `Secret` de Kubernetes llamado `redis-secret`. Esto permite mantener segura la contraseña del sistema de almacenamiento mientras el consumidor opera de forma autónoma dentro del clúster.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kafka-consumer
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: kafka-consumer
  template:
    metadata:
      labels:
        app: kafka-consumer
    spec:
      containers:
      - name: consumer
        image: sergiolarios/kafka-consumer:latest
        env:
        - name: REDIS_HOST
          value: "redis-service:6379"
        - name: REDIS_PASSWORD
          valueFrom:
            secretKeyRef:
              name: redis-secret
              key: password
```

#### kafka-producer-deployment.yaml

El archivo `kafka-producer-deployment.yaml` despliega una réplica del microservicio `kafka-producer` dentro del namespace `weather-tweets`, usando la imagen `sergiolarios/kafka-producer:latest`. Este contenedor está configurado para enviar mensajes a Kafka utilizando la variable de entorno `KAFKA_BOOTSTRAP_SERVERS`, que apunta al servicio interno de Strimzi (`my-cluster-kafka-bootstrap.weather-tweets:9092`), y al tópico `tweets-topic` definido por la variable `KAFKA_TOPIC`. Este productor es responsable de publicar los tuits procesados desde el cliente Go o desde otros servicios hacia el clúster de Kafka.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kafka-producer
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: kafka-producer
  template:
    metadata:
      labels:
        app: kafka-producer
    spec:
      containers:
        - name: producer
          image: sergiolarios/kafka-producer:latest
          env:
            - name: KAFKA_BOOTSTRAP_SERVERS
              value: "my-cluster-kafka-bootstrap.weather-tweets:9092"
            - name: KAFKA_TOPIC
              value: "tweets-topic"
```

#### kafka-producer-service.yaml

El archivo `kafka-producer-service.yaml` crea un servicio de tipo `ClusterIP` llamado `producer` dentro del namespace `weather-tweets`, el cual expone el puerto `50051` del contenedor `kafka-producer` para habilitar la comunicación interna vía gRPC. El selector `app: kafka-producer` asegura que las solicitudes se enruten correctamente hacia el pod correspondiente, permitiendo que otros servicios dentro del clúster, como el cliente Go, puedan enviar datos al productor Kafka mediante llamadas gRPC.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: producer
  namespace: weather-tweets
spec:
  selector:
    app: kafka-producer
  ports:
    - protocol: TCP
      port: 50051
      targetPort: 50051
  type: ClusterIP
```

#### kafka-service.yaml

El archivo `kafka-service.yaml` define un servicio de tipo `ClusterIP` llamado `my-cluster-kafka-bootstrap` en el namespace `weather-tweets`, que expone el puerto `9092` y permite que los productores y consumidores se conecten al clúster de Kafka administrado por Strimzi. El selector `strimzi.io/cluster: my-cluster` asegura que el servicio enrute las solicitudes correctamente hacia los pods del clúster Kafka identificado como `my-cluster`, funcionando como punto de entrada principal para clientes Kafka internos.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: my-cluster-kafka-bootstrap
  namespace: weather-tweets
spec:
  ports:
    - port: 9092
      targetPort: 9092
  selector:
    strimzi.io/cluster: my-cluster
```

#### kafka-topic.yaml

El archivo `kafka-topic.yaml` define un recurso `KafkaTopic` llamado `tweets-topic` dentro del namespace `weather-tweets`, asociado al clúster de Kafka `my-cluster` gestionado por Strimzi. Este tópico está configurado con 3 particiones para permitir procesamiento paralelo de mensajes y una réplica (ideal para entornos de desarrollo). Además, se especifica una retención de mensajes de 7 días (`retention.ms: 604800000`) y un tamaño máximo de segmento de 1 GB (`segment.bytes`), lo que permite controlar la duración y el tamaño de los datos almacenados en el tópico.

```yaml
apiVersion: kafka.strimzi.io/v1beta2
kind: KafkaTopic
metadata:
  name: tweets-topic
  namespace: weather-tweets
  labels:
    strimzi.io/cluster: "my-cluster"
spec:
  partitions: 3
  replicas: 1
  config:
    retention.ms: 604800000  # 7 days
    segment.bytes: 1073741824
```

### RabbitMQ
#### rabbit-consumer-deployment.yaml

El archivo `rabbit-consumer-deployment.yaml` define un Deployment que lanza una réplica del contenedor `sergiolarios/rabbit-consumer:latest` en el namespace `weather-tweets`. Este consumidor está configurado para conectarse a RabbitMQ mediante la variable de entorno `RABBITMQ_HOST`, que incluye la URL completa de conexión al servicio RabbitMQ interno del clúster (`rabbitmq.weather-tweets.svc.cluster.local:5672`). Su función es consumir mensajes desde la cola correspondiente en RabbitMQ para procesarlos o almacenarlos según la lógica definida en el contenedor.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rabbitmq-consumer
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rabbitmq-consumer
  template:
    metadata:
      labels:
        app: rabbitmq-consumer
    spec:
      containers:
        - name: consumer
          image: sergiolarios/rabbit-consumer:latest
          env:
            - name: RABBITMQ_HOST
              value: amqp://user:Gh62vf3qHqIzFoI3@rabbitmq.weather-tweets.svc.cluster.local:5672/
```

#### rabbit-producer-deployment.yaml

El archivo `rabbit-producer-deployment.yaml` contiene dos recursos: un Deployment y un Service. El Deployment lanza una réplica del contenedor `sergiolarios/rabbit-producer:latest` dentro del namespace `weather-tweets`, exponiendo el puerto `50052` y utilizando la variable de entorno `RABBITMQ_HOST` para conectarse al servicio interno de RabbitMQ (`rabbitmq.default.svc.cluster.local`). Este contenedor actúa como productor, enviando mensajes a una cola en RabbitMQ. El Service asociado, también llamado `rabbitmq-producer`, es de tipo `ClusterIP` y permite a otros pods dentro del clúster comunicarse con el productor a través del puerto `50052`.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rabbitmq-producer
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rabbitmq-producer
  template:
    metadata:
      labels:
        app: rabbitmq-producer
    spec:
      containers:
        - name: producer
          image: sergiolarios/rabbit-producer:latest
          ports:
            - containerPort: 50052
          env:
            - name: RABBITMQ_HOST
              value: rabbitmq.default.svc.cluster.local
---
apiVersion: v1
kind: Service
metadata:
  name: rabbitmq-producer
spec:
  selector:
    app: rabbitmq-producer
  ports:
    - port: 50052
      targetPort: 50052
```

### Redis
#### redis-deployment.yaml

El archivo `redis-deployment.yaml` define el despliegue de una instancia de Redis versión 7 en el namespace `weather-tweets`, con una sola réplica. El contenedor expone el puerto `6379`, que es el puerto por defecto para conexiones a Redis, y se identifica con la etiqueta `app: redis` para su posterior descubrimiento mediante servicios de Kubernetes.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:7
        ports:
        - containerPort: 6379
```

#### redis-secret.yaml

El archivo `redis-secret.yaml` crea un `Secret` llamado `redis-secret` en el mismo namespace, donde se almacena de forma segura la contraseña codificada en base64 (`U08xXzFTMjAyNQ==`, que equivale a `SO1_1S2025`). Este recurso puede ser referenciado por los contenedores que necesiten autenticarse con la instancia de Redis, evitando incluir la contraseña directamente en los archivos de despliegue.

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: redis-secret
  namespace: weather-tweets
type: Opaque
data:
  password: U08xXzFTMjAyNQ==  # base64 de "SO1_1S2025"

```

#### redis-service.yaml

El archivo `redis-service.yaml` expone la instancia de Redis mediante un `Service` de tipo `NodePort` llamado `redis-service`, permitiendo el acceso externo al contenedor a través del puerto `31379`, el cual redirige internamente al puerto `6379` del pod. Gracias al selector `app: redis`, este servicio se vincula directamente al pod desplegado por el Deployment de Redis.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: redis-service
  namespace: weather-tweets
spec:
  selector:
    app: redis
  ports:
    - protocol: TCP
      port: 6379
      targetPort: 6379
      nodePort: 31379  # debe estar entre 30000-32767
  type: NodePort
```

### Rust
#### rust-api-deployment.yaml

El archivo `rust-api-deployment.yaml` despliega una instancia del microservicio `rust-api` en el namespace `weather-tweets` utilizando la imagen `sergiolarios/rust-api:latest`, expuesta en el puerto `8080`. Este contenedor se comunica con el servicio Go mediante la variable de entorno `GO_SERVICE_URL`, y se le asignan recursos mínimos y límites definidos para un uso eficiente. Además, incluye un recurso `HorizontalPodAutoscaler` que ajusta automáticamente la cantidad de réplicas entre 1 y 3 según el uso de CPU, escalando cuando la utilización promedio supere el 30%.

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-api
  namespace: weather-tweets
spec:
  replicas: 1
  selector:
    matchLabels:
      app: rust-api
  template:
    metadata:
      labels:
        app: rust-api
    spec:
      containers:
      - name: rust-api
        image: sergiolarios/rust-api:latest
        ports:
        - containerPort: 8080
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "500m"
            memory: "256Mi"
        env:
        - name: GO_SERVICE_URL
          value: "http://go-client-service:8081/input"
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: rust-api-hpa
  namespace: weather-tweets
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: rust-api
  minReplicas: 1
  maxReplicas: 3
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 30
```

#### rust-api-service.yaml

Por su parte, el archivo `rust-api-service.yaml` define un `Service` de tipo `ClusterIP` llamado `rust-api-service` que expone internamente el puerto `8080` del contenedor Rust. Esto permite que otros componentes del clúster, como el Ingress o el cliente Go, se comuniquen con el servicio Rust a través del nombre del servicio DNS sin exponerlo externamente.

```yaml
apiVersion: v1
kind: Service
metadata:
  name: rust-api-service
  namespace: weather-tweets
spec:
  selector:
    app: rust-api
  ports:
  - port: 8080
    targetPort: 8080
  type: ClusterIP
```

## ¿Cómo funciona Kafka?

Apache Kafka es una plataforma distribuida para la transmisión de datos en tiempo real. Su arquitectura permite enviar, almacenar y procesar grandes volúmenes de mensajes de forma eficiente, escalable y tolerante a fallos. En este proyecto, Kafka se utiliza como sistema de mensajería intermedio para el procesamiento de tuits meteorológicos.

Kafka organiza los datos en **topics**, que son canales de comunicación donde los productores publican mensajes y los consumidores los leen. Cada topic puede dividirse en **particiones**, lo que permite distribuir la carga entre múltiples instancias (brokers) y facilitar el procesamiento paralelo. Los **producers** envían mensajes a un topic específico, mientras que los **consumers** se suscriben a esos topics para recibir los datos en tiempo real.

En esta implementación, el microservicio `go-client` actúa como productor, enviando los tuits al tópico `tweets-topic`. A su vez, el microservicio `kafka-consumer` lee los mensajes del tópico y los almacena en Redis para su posterior visualización. Todo esto sucede dentro del clúster de Kubernetes, donde Kafka está desplegado usando Strimzi, una solución que simplifica la gestión de Kafka en entornos Kubernetes.

Kafka garantiza la durabilidad de los mensajes mediante almacenamiento persistente en disco, y permite que múltiples consumidores lean los mismos datos sin afectar la disponibilidad ni la integridad del sistema. Esto lo convierte en una herramienta ideal para arquitecturas orientadas a eventos como la de este proyecto.

## ¿Cómo difiere Valkey de Redis?
Valkey es un fork comunitario de Redis surgido en 2024, impulsado por la comunidad de código abierto tras la adopción de un modelo de licencia restrictiva por parte de Redis Inc. Aunque inicialmente ambos sistemas son funcionalmente equivalentes, Valkey busca mantener una evolución abierta y comunitaria del sistema, garantizando que siga siendo gratuito y libre para todos los usos. Técnicamente, Valkey es compatible con Redis 7.x, lo que permite migrar o utilizar ambos indistintamente en entornos de desarrollo actuales.

## ¿Es mejor gRPC que HTTP?
gRPC y HTTP tienen enfoques distintos y la elección depende del caso de uso. gRPC es un protocolo basado en HTTP/2 que utiliza serialización binaria (Protocol Buffers), lo que lo hace más eficiente en rendimiento, especialmente en arquitecturas de microservicios donde hay muchas llamadas entre servicios. También permite definir servicios con múltiples métodos y soporta autenticación, compresión y streaming bidireccional. En cambio, HTTP (REST) es más simple, ampliamente soportado por herramientas y fácil de depurar, lo que lo hace ideal para APIs públicas o aplicaciones que no requieren alto rendimiento. En este proyecto, se utiliza gRPC entre servicios internos (como Go y Kafka/RabbitMQ) para eficiencia, y HTTP para la interfaz REST que comunica con el exterior (por ejemplo, desde Rust).

## Para los consumidores, ¿Qué se utilizó y por qué?
Para los consumidores de mensajes se utilizaron contenedores personalizados: `kafka-consumer` y `rabbit-consumer`, ambos escritos en Go. Estos servicios se encargan de leer mensajes desde sus respectivas colas (Kafka y RabbitMQ) y almacenarlos en Redis para análisis y visualización posterior. Se optó por Go por su rendimiento, concurrencia nativa (goroutines) y bajo uso de recursos, lo cual lo hace ideal para tareas de consumo continuo y procesamiento ligero en tiempo real. Además, su rápida integración con librerías como `streadway/amqp` y `segmentio/kafka-go` facilitó la implementación.

## Locust

http://34.41.116.132.nip.io/input

`locust -f index.py`