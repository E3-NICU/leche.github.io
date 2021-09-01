## General
This folder contains all the data measured during the project. 
Most of the data is useful, but some was not used.

External temperature refers to the temperature measured with the IR sensor.
Internal temperature refers to the temperature with the multimeter with wire sensor.

## Files
### hospital fridge temp
This file contains a few measurements of 
external milk temperature from the fridge at the hospital.

| name | type | description |
|------|:----:|-------------|
| temp | **float** | The *external* temperature (*C) right after being taken from fridge. |

### pair temp
This file contains measurements of milk warmed up by medical staff, 
and the temperature when it was administered.

| name | type | description |
|------|:----:|-------------|
| warmup | **float** | The *external* temperature (*C) right after warmup. |
| administered | **float** | The *external* temperature (*C) right before administration. |

### staff warmup temp
This file contains the temperatures and metadata 
of milk warmed up by medical staff to be consumed.

| name | type | description |
|------|:----:|-------------|
| id | **unsigned** | Is the same when the containers were together in the microwave, otherwise unique. |
| shift_id | **unsigned** | Unique identifier for shift. |
| shift_time | **unsigned** | Between 0 and 24 indicates the time of the shift. |
| microwave |  **string** | Was it the old, good microwave or the bad, new one. |
| temp | **float** | The *external* temperature (*C) right after being warmed up. |
| volume | **unsigned** | Amount of milk (ml). |
| container | **string** | Container/syringe type. |
| extra_info | **string** | Additional information on measurement. |

## fridge in ex
This file contains the measurements done to correlate internal and external temperature.
All measurements in this file are from containers put in the fridge.

This data will be used to map the temperature decline and to test 
several theories like consistency of warmup and container differences.

| name | type | description |
|------|:----:|-------------|
| container | **string** | Container/syringe type. |
| start_temp | **unsigned** | Measured *internal* temperature (*C) before microwave. |
| volume | **unsigned** | Amount of water (ml). |
| watt |  **unsigned** | Amount of watts used by microwave. |
| seconds | **unsigned** | Time (s) in microwave. |
| room_temp | **unsigned** | Room temperature (*C) measured by both IR and climate control. |
| in* | **float** | *Internal* temperature after x seconds, values range from 30 to 180 in increments of 30. |
| ex* | **float** | *External* temperature, same values as in*. |

## room in ex
Same as **fridge in ex** file but for water that started at room temperature.
This data will probably not be used.

## model 90 / 360
These two files contain measurements for microwave warmup for different volumes.
All measurements in this file are from containers put in the fridge.

It contains measurements for every volume and sensible time in microwave 
to achieve a temperature between 20~38 (*C).

The **model 90** file contains measurements done at 90 watts with the 20ml spuit.
The **model 360** file contains measurements done at 360 watts with the 60ml spuit.


| name | type | description |
|------|:----:|-------------|
| seconds | **unsigned** | Time (s) in microwave. |
| volume | **unsigned** | Amount of water (ml). |
| start_temp | **unsigned** | Measured *internal* temperature (*C) before microwave. |
| internal_temp |  **unsigned** | Measured *internal* temperature (*C) after microwave and shaking. |
| external_temp | **float** | Measured *external* temperature (*C) after microwave and shaking. |