# Ommatidia: Improving generalizability for pupil detection algorithms in head-mounted eye tracking
## Introduction
Eye-tracking is a ubiquitous technology to record, assess, and evaluate eye movements, gazes, and pupil reactions in a magnitude of different research fields [@punde2017]. Historically, corresponding pipelines aimed for otherwise hardly measurable quantities within psychophysiological studies [@steinhauer2022]. However, the technology starts to be more widely applied as an invaluable aid for optimized human-computer interaction and experiments in more applied scientific areas like medicine [@larrazabal2019]. Consequently, the growing demand for objective measurements fuels the development of required technology in research and industry. With reduced costs, fewer complexities, and improved accuracy, the technology will presumably get even more prevalent in the future [@stein2021].

Stationary eye trackers are generally researched for scientific purposes since the late 19th century [@cognolato2018]. More recently, head-mounted video oculography evolved as a valuable alternative allowing former impossible experimental setups [@cazzato2020a]. Fueled by the minimalization of optical technology, cameras inbuild in glasses-like devices or virtual reality headsets record the eyes constantly with non-visible near-infrared light. Unlike the popular yet proprietary devices used before, the complete process of recording is transparent and re-evaluable accordingly to advances in analysis methods. Additionally, affordable off-the-shelve technology is available or could straightforwardly tailormade [i.e. @zandi2021; @stengel2015]. Consequently, the technology directly supports the four principal components of open science [@vicente-saez2018].

The downside of such great flexibility and archivable tailor-made solutions is the responsibility to tune those magnitudes of factors influencing the actual recording process. Besides the physical camera setup, the pupil detection algorithms represent the primary component of the eye-tracking pipeline. Despite the apparent simplicity of detecting the corresponding blob of pixels in a near-infrared image, this task is challenging and far from being solved. Factors such as contact lenses, dust, makeup, and other external factors induce further complications in the measurement process [@tonsen2016]. Given the magnitude of available algorithms with different advantages and disadvantages, the researcher must choose the method and associated hyperparameters optionally matching its aim [@chen2019]. Guiding those choices given reliable evidence and ease of application define the aim of this publication.

## Challenges with pupil data
Due to various reasons, a decent recording and appropriate analysis of eye-tracking data is not a trivial task [@orquin2018]. However, those well-known complexities lead to a considerable collection of published knowledge and pieces of advice in literature [i.e. @steinhauer2022; @godfroid2020]. Those published insights into possible recording setups and devices [i.e. @stein2021; @cognolato2018], optimization strategies regarding data quality [i.e. @kret2019], and proper analysis methods [i.e. @vanrij2019] allow valuable hints towards the proper use of the powerful methodology. Consequently, any researcher striving to apply it could build upon a vivid ecosystem of scientific work.

Supporting the applied researcher in choosing a particular pupil detection algorithm for the measurement pipeline, some evaluation surveys like @fuhl2016b try to compare and rate those algorithms. Using similar evaluation criteria as the developers of the algorithms, the authors provide valuable additional evidence regarding the performance without any bias slipping in unintentionally. In general terms, those reviews document the expected continous improvement of the algorithms due to rising complexity and available data. Still, the ways of analyses do not vary significantly to the original papers. Consequently, the performance of the algorithms is assessed upon multiple datasets.
 
Different datasets for such an evaluation exists due to the diversity of potential recording designs [@fuhl2021a]. Most of those were used for training or evaluating a pupil detection algorithm. Consequently, they differ in the types of labels associated with the sample depending on the three major types of algorithms.
Most fundamental is the detected pupil center within the image [@fuhl2016b]. Given these two-dimensional coordinates and a (commonly estimated) three-dimensional model of the eye, subsequent procedures can calculate the corresponding rotation of the eye [@santini2019]. 
If pupil size matters like in pupillograpy, the algorithms typically yield the best-fitting ellipse enclosing the pupil [@zandi2021]. The output contains the size of the major and minor axis and its rotation in addition to the two-dimensional position.
The most versatile representation utilizes a segmentation map over the complete sample [@fuhl2021a]. This segmentation mask corresponds to a binary mask where only the pupil is indicated. Partially, those algorithms yield similar data for other components of the eye like the iris and sclera, too. In theory, such encoding would even permit using pupils partially hidden due to eyelids or blinks.
Crucially, these three different annotations create a hierarchy regarding their information. While the segmentation mask contains most information, reducing it to an ellipse or even the pupil center is feasible. Given large annotated datasets, researchers rely on the fact algorithms performing well on them will generalize well on unseen setups and subjects, too.

The concept of generalizability is crucial for such an assumption [@kothari2022]. However, there is no guarantee sufficient performance on other datasets will lead to sufficient performance on the custom setup, too. Instead, different authors like @cazzato2020a highlighted the importance of custom adaptations required for sufficient performance. These complexities result from a magnitude of different aspects:

-   The recorded samples differ significantly based on the position of the camera, its resolution, and distance [@niehorster2020]. Consequently, those of different recording setups are not directly comparable. Given the non-linear transformation of the pupil once viewed from larger eye angles, the performance of the algorithms might be significantly more challenging [@petersch2021].
-   The algorithms often require the setting of hyperparameters depending on the samples. Many of those are directly related to semantical meaning and tailored to the specific position of the camera. While reusing those values published may be sufficient if the setups are similar enough, getting more suitable detections will likely depend upon tuning those values.
-   The population of the subjects may differ considerably. Especially in the medical context, specific correlated phenotypes may seriously hinder the detection rate. Published work like @kulkarni2021 systematically evaluating induced bias in pupil detection is still scarce. Furthermore, even within the general population, such challenges are well-documented [@fuhl2016b]. As an example, measuring specifically subjects with contact lenses requires detectors to perform well even in this specific condition without inducing any bias.
-   The metrics used for performance evaluation differ significantly between studies. Often, they were chosen for optimally assessing a specific dataset or use case. For instance, the evaluation paper by @fuhl2016b used a threshold of five pixels for classifying the detection of the pupil center inside a sample as correct. Given the tested datasets, this is a thoroughly sound decision. However, samples with significantly different resolutions due to another camera setup require the adoption of such concepts.

Given those complexities, the evaluation of pupil detection algorithms must consider their context. Claiming generally superior performance in direct comparison with all competitors appears challenging. Consequently, custom considerations and evaluations in the application of pupil detection algorithms remain necessary.

Besides the hypothetical overall performance, other key concepts influence the decision in favor of one or another pupil detection algorithm. Those other factors arise analogously to other use cases of machine learning where softer concepts start to play a role, too.
Transparency is an example of such an additional dimension [@cazzato2020a]. When applied in sensible medical areas, understanding the reasons regarding a specific output might be required.
Another reason is suitable licensing. Some algorithms like @santini2018a are licensed for non-commercial usage only. Even in academia, such a license would be incompatible with classical open-source licenses like the GPL. Their requirement regarding re-licensing under the same terms would probit the proper publication required for transparent science. 
Considering those facts further complicates the appropriate choice of a pupil detector besides the assumed detection performance.

Considering all those factors when choosing one out of the many available pupil detection algorithms remains a challenge. Choosing whether to stick to a popular default or choose and tune the algorithm for optimal performance might depend on the precise specification of the project. However, even the soundest knowledge regarding the literature might not be sufficient in the latter case. Consequently, empowering an applied researcher to select and fit the best-fitting detector for the research is the primary contribution of this paper.

## Unifying framework for the assessment of pupil detection algorithms
The aim of this publication is the empowerment of researchers from various scientific disciplines toward the independent assessment of pupil detection algorithms within their experiments. The corresponding procedure of assessment must be as inclusive as possible. It should be easily usable and must not require detailed knowledge regarding the details of method and implementation. Under the premise, the researcher should be able to optimize the measurements of their raw experimental data for the sake of more sustainable science.

To enforce those central concepts, we defined the required constraints any proposed toolkit must fulfill to be applied effectively in academic practice. Naturally, most of the complexities associated with the evaluation should be handled automatically without manual intervention. However, establishing a sustainable and reusable utility requires a more precise definition regarding the underlying constraints.

- The proposed framework must be as flexible as possible regarding the hardware and software environment for its execution. It must not be required to run on remote servers but allow for full offline use. Allowing for widespread use in all countries, it should not require a specific commercial system that might be inaccessible due to license regulations or fees. Additionally, applied researchers should be able to use it directly on the system already employed for the other parts of the conducted experiment. Avoiding the copies of raw data in different systems may help prevent data loss, respect privacy constraints, and simplify the general knowledge. Consequently, the toolkit must support recent hardware both with UNIX-based operating systems often licensed as Open Source software and with the popular yet proprietary Microsoft Windows.
- Once a suitable system is available, setting up the toolkit should be as straightforward as possible and not require advanced knowledge. While this seems like a trivial task, it is seriously hindered by the different pupil detection algorithms. Available implementations depend upon various programming languages and require a magnitude of different libraries and build tools. Comparable to many of the scripts published in scientific publications, installing all those dependencies is often challenging and time-consuming. Consequently, a manual setup must not be required. Therefore, the results of the assessment are faster and easier to archive.
- The proposed toolkit must be scalable given the large numbers of samples in recent datasets, the variety of different algorithms, and the total sum of tunable hyperparameters. Luckily, the independence of algorithms and samples leads to easily archivable parallelization of the detections. Consequently, the toolkit should be able to benefit from a single machine, multiple virtual machines, and even a cluster of physical devices. Such an ability allows the most efficient exploration of the extensive search space given the available computational resources.
- The toolkit should be modular and base upon existing standards. Sticking to established best practices simplifies the support and supports sustainable development. Additionally, those standards allow the re-use of individual components within the system without requiring the usage of the system as a whole. As an example, after a pupil detector was selected successfully its implementation could be used inside the final experiment, too.
- The toolkit should be easily adaptable not only for those primarily employing pupil detection algorithms but for those developing them. Given the larger number of available competitors, the developers of a new detector may gain important insights for their optimization. Additionally, the risk of involuntary selection bias within the associated publication might be reduced given the strictly reproducible nature. As such, the toolkit may contribute to the sustainable development of the methodological area, too.

Given those constraints, a setup based upon microservices and distributed computing appear appealing. The rest of the section will describe the methodological and technical details of the modular, scalar, and effortlessly adaptable toolkit.

### Selection of the pupil detection algorithms
Obtaining an appropriate and meaningful comparison of pupil detection algorithms requires a sufficiently large set of corresponding implementations. Consequently, we conducted an extensive literature analysis for obtaining the currently available state-of-the-art. The foundation of this investigation was the extensive review be TODO. Based on their selected paper, follow-up publications, and own research, we analyzed a collection of TODO algorithms specified in table TODO. Please note this collection might not be fully conclusive but contains most of those algorithms currently used by researchers. Afterward this search, we evaluated the inclusion of each algorithm individually according to some general rules.

- The published algorithms must enclose associated implementations. In theory, it might be theoretically possible to replicate a possible source code given the textual description only.  However, many of the fundamental computer vision algorithms differ slightly in their implementations and defaults regarding hyperparameters. Consequently, we only included those algorithms behaving exactly as the original authors intended.
- In general, we do not enforce any dependencies or programming languages. However, allowing the corresponding software to be run on UNIX-based systems prevents the usage of proprietary components. Consequently, implementations only available as compiled Microsoft Windows library without any associated source code were excluded. The same holds for algorithms written in scripting languages requiring a paid license like MATLAB.

All implementations matching those two assumptions were included within the toolkit. Unlike the comparison by TODO, we did not require those algorithms to run in real-time as offline analyses might be possible given the specific experimental setup. To the best of our knowledge, the TODO included algorithms represent the largest collection available for evaluation purposes.

### Architecture and design of the toolkit
The proposed toolkit employs a microservice architecture for improved robustness, scalability, and compatibility. Consequently, the minimal building blocks correspond to fully autonomous containers. Those services represent a remarkably lightweight kind of visualizations by using the existing kernel of the operating systems instead of a complete simulation as in virtual machines. Each container includes all the essential dependencies required by one particular implementation of a pupil detection algorithm. This includes all the software components required to build and execute the code, the dependencies upon other libraries and data, and the source code itself. Consequently, they encapsulate all those parts of the environment a scientist would otherwise install and configure manually. Additionally, the content of the containers is licensed accordingly to their included code due to their autonomy. As changing one container does not affect the other, such a service-oriented architecture appears well suited to represent the variety of different pupil detection algorithms.

The design of the proposed architecture is inspired by the current trends in the industry. Besides the advantages of the autonomous services, their collective deployment is often a key reason for establishing similar architectures. By now, different container runtimes executing the individual containers are available and support diverse conditions and demands. The concept of hardware has only limited importance in similar setups. The container runtime does not differentiate between physical hardware available at the researcher's desk or virtual machines provided by the institute's servers. Even a deployment across a cluster of different virtual machines commonly termed "cloud" is achievable. Consequently, individual experiments could use all the available resources and scale almost linearly with the amount of the latter.

Services delivered in the form of containers are commonly depending on well-documented interfaces for interaction. The underlying implementation is entirely opaque and treaded as a black box. Consequently, the communication and transmission of data require a well-established network stack. Application interfaces based upon the Hypertext Transfer Protocol (HTTP) are popular due to their widespread use and the broad availability of corresponding server and client libraries for most programming languages. Additionally, using this technology to design interfacing following the Representational state transfer (REST) style allows human-understandable communication with the containers. Ensuring compatibility and interoperability, standards like OpenAPI allow the automatical discovery, querying, and testing of available resources. Following those inofficial standards, we exposed a unified application interface for all pupil detectors allowing their flexible use in numerous contexts.

Given those containers with uniquely defined REST application interfaces, most programming languages support sending of samples and receiving the estimated pupils. Consequently, any researcher could utilize a container runtime and develop evaluation procedures individually. However, most of the associated operations for managing the lifecycle of containers and processing their input and output appear repetitive. Consequently, the proposed toolkit includes an optional management tool handling those procedures in a performance-optimized way. Controlled by a command-line interface and a comprehensible configuration file, this software allows for scalable and memory-preserving bulk processing of multiple samples. Afterward, the obtained pupil estimates encoded as JSON are the foundation for the specific evaluations conducted by the researcher.

## Example
TODO

## Discussion
TODO

## References