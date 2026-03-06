# 🐾 Pet Health Vault - Solana

Este proyecto es un sistema de **expedientes médicos para mascotas** en la red de Solana. La idea es tener un registro seguro, inmutable y descentralizado de la salud de nuestras mascotas que no se pueda borrar.

## 🛠️ Tecnologías y Herramientas
* **Lenguaje**: Rust.
* **Framework**: Anchor.
* **Entorno**: Solana Playground.

## 📂 Archivos del Repositorio
* **`lib.rs`**: Código principal del contrato inteligente en Rust.
* **`PetHealth_test.ts`**: Script para probar que el registro y las consultas funcionan bien.
* **`Reporte_PetHealth.md`**: Explicación técnica sobre el almacenamiento y el uso de PDAs.

## 🚀 Instrucciones de Uso

### 1. Registrar una Mascota
Ejecutar la función `registrarMascota` para crear el expediente.
* **Semillas (Seeds)**: Se requiere la cadena exacta `"expediente"` y la `publicKey` del dueño.
* **Nota**: Un error en el nombre de la semilla (ej. "expediene") causará un error de `seeds constraint violated`.

### 2. Agregar al Historial Médico
Usar la función `agregarConsulta` para subir información al historial médico.
* **Datos**: Se guarda el diagnóstico, tratamiento y el costo de la consulta.

## 📊 Verificación de Resultados
Los datos se almacenan en la blockchain y pueden ser consultados en la sección de **Accounts**. Un expediente exitoso se ve así:

```json
{
  "owner": "8r1YagDooAgiqAsFt7qT7V72vtwzJr6bcngzpQ9EzEdw",
  "nombre": "sol",
  "raza": "husky",
  "historial": [
    {
      "diagnostico": "Revisión general",
      "tratamiento": "Ninguno",
      "costo": 100
    }
  ]
}
