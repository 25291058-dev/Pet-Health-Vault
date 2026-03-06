# Pet Health Vault - Solana

Este proyecto es un sistema de expedientes medicos para mascotas en la red de Solana. La idea es tener un registro seguro que no se pueda borrar.

## Archivos del Repositorio
* lib.rs: Codigo principal del contrato en Rust.
* PetHealth_test.ts: Script para probar que el registro y las consultas funcionan bien.
* Reporte_PetHealth.md: Explicacion tecnica sobre el almacenamiento y el uso de PDAs.

## Instrucciones
1. Ejecutar la funcion registrarMascota para crear el expediente.
2. Usar agregarConsulta para subir informacion al historial medico.
