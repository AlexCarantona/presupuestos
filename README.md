# Presupuestos
Este es un proyecto personal para practicar tanto Rust como ejercicios de contabilidad.
El programa pretende servir para crear, interpretar y corregir asientos contables, ateniéndose al [Plan General de Contabilidad](https://www.boe.es/buscar/act.php?id=BOE-A-2007-19884) actualmente en vigor en España.

## Características principales
El programa se sirve fundamentalmente de la línea de comandos y de la lectura de una serie de archivos y carpetas que deben estar en el mismo directorio de ejecución.
Empieza por cargar automáticamente todas las cuentas previstas en el PGC (ya están incluidas en el binario), que se pueden utilizar mediante códigos inmediatamente.

### Balance de situación
Puede crear un balance de situación inicial si existe un documento llamado **balance_inicial.txt** en el mismo directorio. La estructura es la siguiente:
```
<Código de cuenta> <Saldo>
```
Para facilitar la composición del archivo, se pueden incluir encabezamientos, comentarios... El programa obviará cualquier línea de texto que no sea exclusivamente como la anterior.

### El Libro Diario
El Libro Diario es una secuencia de asientos, almacenados en archivos de texto plano individuales dentro de la carpeta **diario**. Estos archivos se nombran mediante un código único, que se forma del siguiente modo: <FECHA(YYYYMMDD)><Nº de asiento del día>.

En su interior, se organizan así:

```
<Descripción del asiento>

DEBE
<Código de cuenta> <Importe>
<Código de cuenta> <Importe>

HABER
<Código de cuenta> <Importe>
<Código de cuenta> <Importe>

```