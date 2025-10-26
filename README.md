# libfhir

**libfhir** is a library to provides [FHIR Resources](https://www.hl7.org/fhir/resourcelist.html) with useful API's
that aims at reducing complexities and ease of use for consumers of those resources. The latter are infamous for being 
over complex, and unfortunately, all the existing solutions tends to be either a generic implementation of the standard,
or type interfaces that are very difficult to work with. We are not attempting to create a "one solutions fit all", but 
taking an approach of adding actual features that are helpful in real world scenarios (and sometimes too vague in the standard iteslf).


## Goals 

We are aiming to implement [Release 4](https://hl7.org/fhir/R4/resourcelist.html) of the standard. Since this is the 
revision that is being adopted by countries as a first edition. Also,important extension profiles such as [mCSD](https://profiles.ihe.net/ITI/mCSD/index.html) are 
actually still on R4. 

Once the Rust implementation is complete, we want to port those functions to ideally **Python** and **JavaScript/TypeScript**. 

## Status

At this moment, we are currently in a Proof of Concept/exploration phase. However, we are busy with [Release 4](https://hl7.org/fhir/R4/resourcelist.html) of the standard. And would be working on [mCSD Resources](https://profiles.ihe.net/ITI/mCSD/volume-1.html#1-46-mobile-care-services-discovery-mcsd) as a baseline. 


## Contributing to the project

TBA
