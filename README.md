# Ommatidia: Improving generalizability for pupil detection algorithms in head-mounted eye tracking
## Introduction
Eye tracking is a ubiquitous technology to record, assess and evaluate eye movements, gazes, and pupil reactions in a magnitude of different research fields. In research as in industry, the growing demand for measurement for widely different areas. Besides experiments aiming for otherwise hardly measurable quantities like attention or memory, the technology is applied as a product in areas such as human-computer interaction and ophthalmology. With reduced costs and improved accuracy, the technology is likely to become even more popular in future.

While eye tracking with stationary devices is generally available since the 1960s(?), head-mounted video oculography rises to a new standard. Fueled by the minimalization of camera technology, cameras inbuild in glasses or virtual reality headsets constantly recording the eyes with non-visible near-infrared light. Unlike in case of proprietary devices, the complete process of recording is transparent and re-evaluable accordingly advances in technology. Affordable off-the-shelve technology is available or could straightforwardly tailormade. Consequently, the technology directly supports and allows open science.

The downside of such great flexibility and archivable tailor-made solutions is the responsibility to tune those magnitude of factors having influence on the actual recording. Beside the camera setup, the pupil detection algorithms represent the primary component of the eye tracking pipeline. Despite the apparent simplicity of detecting the corresponding blob of pixels in a near-infrared image, this task is challenging and far from being solved. Factors such as contact lenses, dust, makeup, and other external factors induces further complications. Given the magnitude of available algorithms, the researcher must choose the method and hyperparameters optionally matching its aim. Guiding those choices given reliable evidence it the aim of this publication.

## Challenges with pupil data
The well-known complexities related with eye tracking data lead to many available information and advices in literature. The published details regarding the possible recording setups, data quality, and analysis method allow useful hints towards correct use of the powerful methodology. Consequently, researcher can build upon a vivid ecosystem of scientific work.

Regarding the choice of a particular pupil detection algorithm, existing publications strive to compare and rate those according to one another. In general, the presented results show the continuous improvement over time as expected with the raising power and complexity of the approaches. Consequently, the authors provide valuable additional evidence regarding the performance without any potentially bias which might unintentionally slip in when publishing an algorithm. However, the evaluation criterion does not significantly differ between most reviews and the original work: The performance of the algorithms is evaluated on multiple datasets.

Given the straightforward recording setup and the diversity of setups, various datasets exist for evaluation purposes. Those datasets resemble each other on the first glance: They all contain image samples with annotation regarding the two-dimensional representation of the pupil within. While often published besides a newly proposed detection algorithms, some of those explicitly aim to be foundation of such evaluation. Given the large amount of data, researchers rely on the fact algorithms performing well on it will generalize well on unseen setups and subjects, too.

While the concept of generalizability is central for this claim, there is no guarantee sufficient performance on other dataset will lead to sufficient performance on the custom setup, too. A magnitude of factors may hinder these plans:
-	The recording setups differ significantly in the position of the camera, its resolution and distance. Given the non-linear transformation of the pupil once viewed from larger case angles, the complexity may differ significantly.
-	The algorithms often require the setting of hyperparameters depending on the samples. While reusing those publicated may be sufficient, getting more suitable detections is likely to depend upon tuning those values.
-	The population of the subjects may differ considerably. Especially in the medical context, specific phenotypes may seriously hinder the detection rate. Additionally, even within the general population such challenges are reported. As an example, measuring specifically subjects with makeup requires detectors performing well even in this specific condition without inducing any kind of bias.
-	The metrics used for evaluation differ between studies and are often tailored to specific dataset. For instance, [TODO] used a threshold of five pixels for classifying the detection of the pupil center inside a sample as correct. Consequently, samples with different resolutions due to another camera setup require adoption of such concepts.

Given those facts, deriving the claim of generally superior performance in direct comparison with competitors appears challenging.

Beside the general performance, a hypnotized overall performance is not the only concept guiding the discussion in favor of one favor of another. As similar with other methodology in machine learning, more soft concepts play a role, too.  Transparency might be such a necessarily when the detectors are applied in sensible medical areas. Given the large number of algorithms which are licensed only for non-commercial usage, using such code might be forbidden in custom software with incompatible Open-Source licenses. Considering those facts further complicates the appropriate choice of a pupil detector besides generalizability.

Given those magnitudes of factors to be considered when applying the pupil detection algorithms in practice, especially larger scale experiments may profit from tuning. Empowering a practitioner to fit and select those detectors to the setup with as few efforts as possible is the primary contribution of this paper.

## Unifying framework for the assessment of pupil detection algorithms
The aim of this publication is the empowerment if the practitioners from various disciplines towards the independent assessment of pupil detection algorithms. Without deeper knowledge regarding the codebase, having various state-of-the-art variants must be present. Under the premise, they should be able to select the optimal detector with the needs with the minimal effort required to bring the theory into practice. The usefulness of the framework will then be demonstrated on a real example.

We propose a framework inspired by the current developments regarding microservices and distributed computing. These design criterions are enforced to provide a reasonable foundation for individual experiments.
-	The framework must be as hardware independent as possible. Applied researchers should be able to use it on the system already used for the other parts of the experiment conducted. Consequently, support both on UNIX-based systems like Ubuntu or macOS and Microsoft Windows is required. Limiting the necessarity for changes regarding the system, it should induce error-robustness and simplify the usage in practice.
-	Providing a straightforward experience by this is only one part of the general strategy. In the best case, the researcher should only require installing a single software instead of all the different algorithms, their dependencies and build tools. Given those simplicity, gained results should be faster and easier archivable.
-	Given the large number of samples in recent datasets, the mere number of different algorithms, and the number of tunable hyperparameters, the proposed framework must scalable. Given the resources available, it must be able to benefit from a single machine, multiple virtual machines and even a cluster of physical devices. the independence of algorithms and samples leads to easily archivable parallelization of the detections. Consequently, a broader exploration of the search space with algorithms and hyper parameters is possible.
-	The framework should be modular and base upon existing standards. The corresponding documentation does simplify the support of the individual components. More crucially, it does allow the re-use of the system without the necessarity for the usage of the system as hole. For example, after the successful selection of an implementation, such implementation could be used inside the final experiment, too.
-	The framework should be easily adoptable not only for those using pupil detection algorithms but for those developing them. Given the larger number of competitors, the authors may gain important insights for their optimization. Additionally, the risk of involuntary selection bias is reduced given the strictly reproducible nature of the framework. As such, the it may contribute to a sustainable development of the scientific area.

Given those constraint, a Container-based setup appears appealing. The rest of the section will describe the details of those modular, scalar and easily adoptable framework.

### Selection of the pupil detection algorithms
For a mostly completely collection of state-of-the-art algorithms, we utilize the extensive overview by TODO. The fundamental idea is the assumption of as few assumptions as possible. Realtime processing as sometimes enforced is explicitly not a constraint. Instead, we only enforce those practices strictly necessary for most open and reproducible software in science:
-	We do not enforce any specific programming language. However, the only requirement is the general usability of the language on the computer of the practitioner without paying license fees. Therefore, for example MATLAB-based pupil detectors are excluded.
-	We do not enforce any specific hardware setup or operational system. The only assumption is the general runability of Linux-based container based upon the software “Docker”. This constraint is fulfilled on Linux, Microsoft Windows and macOS based systems.
-	We do not restrict ourselves to pupil detection algorithms from recent research. Instead, we consider open software explicitly designed for this task, too. Given its growing importance in the field, we consequently consider the popular Pupil software, too.

Under those considerations, we included TODO algorithms into our framework. To the best of our knowledge, this is the largest available group of algorithms for the detection of pupils directly applicable to custom datasets.

### Design of the framwork
The minimal atom of the framework are the containers. Those fully independent components include the actual implementation of the pupil detection algorithm. They encapsulate all those parts of the environment the scientists would otherwise require installing themselves on their systems. This includes all the software required to build and execute the code, the dependencies upon other code and data, and the source code itself. Consequently, the programming language and further context are not visible nor of any particular interest. Given their re-usability and independence, the containers are licensed accordingly to their included code and accessibly on their own.

Given the missing information regarding the implementation, rather universal standards are required alternatively for communication and transmission of detections. As a well-established standard, we developed a REST interface for all the pupil detectors included. The usage of HTTP does not only allow the usage of a variety of tools developed for this de-facto standard for the communication with microservice. Furthermore, it allows human-understandable and transparent communication with the algorithms.

-	Minimal component: Each pupil detector in a (Docker) container
o	Those containers: Exactly the environment a scientist would to prepare themselves
o	For some programming language like C++: Re-use of a language-specific library
o	All independent of each other, no communication  Each combination of hyperparameter is different
-	Language-agnostic REST API: Talk to the implementation without knowing the details
o	API defined in a Standard: Open API
o	HTTP: Standard all over the web
o	Human readable: Scientific may even call themselves
-	Runtime for container: Docker
o	Popular and well-established ecosystem 
o	Ability to run multiple container at once
o	Ability to use multiple hosts parallel to each other
	No requirement for local vs remote development
-	Custom software for gluing the components together
o	Configuration in a format like TOML
o	Output: CSV

## Example
## Discussion
