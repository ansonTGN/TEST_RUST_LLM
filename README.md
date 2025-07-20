# 🦀 multi_llm_client · Cliente Rust para LLMs vía HTTP

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Framework-Axum-4B275F.svg)](https://docs.rs/axum/latest/axum/)

---

## 🚀 Descripción

Este proyecto es un **servidor web en Rust** construido con [Axum](https://github.com/tokio-rs/axum) que permite enviar *prompts* a modelos de lenguaje (LLMs) alojados en servicios como [Ollama](https://ollama.com/), y recibir sus respuestas de forma estructurada.

Además, proporciona endpoints de estado y sirve un frontend web básico desde `/public`.

---

## 📚 Endpoints disponibles

| Método | Ruta             | Descripción                                               |
|--------|------------------|-----------------------------------------------------------|
| `GET`  | `/`              | Sirve el archivo `index.html` desde el directorio `public`. |
| `GET`  | `/health`        | Comprueba si el servidor está activo (200 OK).           |
| `GET`  | `/status`        | Devuelve un JSON con el estado del servidor.             |
| `POST` | `/api/prompt`    | Envía un prompt a un LLM remoto vía HTTP.                |
| `GET`  | `/api/models`    | Obtiene los modelos disponibles desde un servidor Ollama.|

---

## 🛠️ Instalación y ejecución

1. **Clonar el repositorio:**

```bash
git clone https://github.com/tu-usuario/multi_llm_client.git
cd multi_llm_client
````

2. **Instalar dependencias (si es necesario):**

```bash
cargo build
```

3. **Ejecutar el servidor:**

```bash
cargo run
```

📍 El servidor quedará escuchando en:

```
http://127.0.0.1:8080
```

---

## 🧪 Ejemplo de uso (API `/api/prompt`)

### Solicitud `POST`:

```json
{
  "url": "http://127.0.0.1:11434",
  "model": "gemma3:12b",
  "prompt": "¿Cuál es la capital de España?"
}
```

### Respuesta esperada:

```json
{
  "response": "La capital de España es **Madrid**."
}
```

---

## 🧩 Dependencias principales

* [`axum`](https://docs.rs/axum) – Framework web asíncrono en Rust.
* [`reqwest`](https://docs.rs/reqwest) – Cliente HTTP robusto.
* [`tokio`](https://tokio.rs) – Runtime asíncrono.
* [`serde`](https://serde.rs) – Serialización y deserialización de datos.
* [`futures-util`](https://docs.rs/futures-util) – Manejo avanzado de streams.
* [`tokio-util`](https://docs.rs/tokio-util) – Adaptadores útiles para Tokio.

---

## 📁 Estructura del proyecto

```
multi_llm_client/
├── src/
│   └── main.rs           # Código principal del servidor
├── public/
│   └── index.html        # Interfaz web frontend
├── Cargo.toml            # Configuración de dependencias
└── README.md             # Este archivo
```

---

## ⚖️ Licencia

Este proyecto está licenciado bajo la licencia MIT. Ver el archivo [LICENSE](LICENSE) para más detalles.

---

## 👨‍💻 Autor

Desarrollado por **Ángel A. Urbina**, 2025.

---

## 🙋‍♀️ Contribuciones

¡Contribuciones, *issues* y *pull requests* son bienvenidos!

---

## 🌐 Recursos relacionados

* [Ollama](https://ollama.com/) – Servidor local para modelos de lenguaje
* [Axum](https://docs.rs/axum) – Framework web asíncrono en Rust
* [OpenAI API](https://platform.openai.com/docs/api-reference)

