## Components
This folder contains the components for the website. 
These should be changed.

## Models
The model folder contains a few files, the most important one is **base**. 
This contains the trait definition for a model used by the GUI. 
A trait can best be compared to a java 'interface' orr c++ concepts.
This trait required you to define two functions forr a type to be a model:
- calc_seconds_and_watt
- calc_expected
A base template for a future model can be found in the **future.rs** file.

```Do not forget to change the model used by the UI in the overview page around line 50```

## Pages
This folder contains the pages for the website.
It currently consists of:
- overview page
- info page
These should be changed.

## Constants
This file contains all constants for the models and UI. 
When a new model is made or slider values/defaults 
need to be changed, this is the place.
