const translations = {
    en: {
        subtitle: "Double Degree Student at Mines Paris PSL and Universidad Nacional de Colombia",
        nav: {
            profile: "Profile",
            education: "Education",
            experience: "Experience",
            skills: "Skills",
            projects: "Projects",
            achievements: "Achievements",
            volunteering: "Volunteering",
            hobbies: "Hobbies",
            pdf: "PDF Version"
        },
        section: {
            profile: "Profile",
            education: "Education",
            employment: "Employment History",
            skills: "Skills",
            projects: "University Projects",
            achievements: "Achievements",
            volunteering: "Volunteering",
            hobbies: "Hobbies"
        },
        profile: {
            text: "Computer Engineering student with experience as software engineer and with a strong passion for mathematics and computer science. Currently pursuing an international double degree between Universidad Nacional de Colombia and Mines Paris PSL. Always looking to learn, build meaningful tools, and be part of projects that make an impact. I adore AI and Machine Learning!"
        },
        education: [
            {
                school: "Mines Paris PSL",
                date: "Aug 2025 – Jul 2027",
                degree: "Master of Science in Engineering, Computational and Applied Mathematics"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "Oct 2021 – Sep 2027",
                degree: "Bachelor's degree in Computer Engineering"
            },
            {
                school: "Czech Technical University in Prague",
                date: "Nov 2025",
                degree: "ATHENS Programme - Probabilistic Methods in Combinatorics"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "Apr 2025 – Aug 2025",
                role: "Software Engineer – Miami, United States (Remote)",
                tasks: [
                    "Developed and maintained tools for dataset creation, preprocessing, and quality assurance for video-based computer vision models.",
                    "Implemented automated workflows to convert, validate, and debug annotations across formats (YOLO, COCO) to support training pipelines.",
                    "Assisted in diagnosing and resolving data integrity and labeling issues from the internal annotation platform.",
                    "Contributed to model training experiments (e.g., RT-DETR) to verify dataset correctness and improve training reliability."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Nov 2024 – Apr 2025",
                role: "Software Engineer – Bogotá, Colombia (Remote)",
                tasks: [
                    "Participated in developing three web applications: two for automating geospatial file processing in civil engineering consulting (backend role), and one for project management and cost estimation (full-stack role).",
                    "Designed and implemented user interfaces, cost automation features, and data integration workflows with external APIs.",
                    "Developed algorithms for external data handling, file parsers, and authentication systems.",
                    "Integrated and managed MySQL/PostgreSQL databases, applying clean architecture principles for scalable and maintainable code."
                ]
            },
            {
                company: "Intelligent Systems Lab (LISI)",
                date: "Aug 2023 – Present",
                role: "Undergraduate Student Researcher – Bogotá, Colombia",
                tasks: [
                    "Active member of research group, participating in seminars on artificial intelligence and collaborative projects with undergraduate and graduate students.",
                    "Prepared and presented materials for seminars on deep learning techniques and architectures (CNNs, VAEs, Transformers, etc.).",
                    "Contributed to the formulation of proposals submitted to Minciencias Calls 950 and 966, integrating artificial intelligence and virtual reality for environmental education."
                ]
            }
        ],
        skills: [
            { category: "Languages:", list: "Spanish (Native) • English (C1) • French (B2) • Portuguese (Conversational)" },
            { category: "Soft Skills:", list: "Teamwork • Problem-solving • Adaptability • Communication • Creativity • Critical thinking" },
            { category: "Programming:", list: "Python (FastAPI, Pandas, NumPy, PyTorch, TensorFlow, etc.) • Julia • Java • C/C++ • Typescript (React) • SQL (PostgreSQL, MySQL, Sqlite) • No SQL (Redis, MongoDB)" },
            { category: "Technical Tools:", list: "Git • AWS • Azure • GCP • Jira • Wolfram Mathematica • LaTeX (Overleaf) • Microsoft Office" }
        ],
        projects: [
            {
                title: "Automated Pipeline for Gaussian Splatting Model Generation",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Developed an immersive virtual exhibition of geological artifacts using Gaussian splats to enhance reflectance, translucency, and realism.",
                    "Built an automated pipeline integrating AWS (S3 and Lambda), RealityScan and LightField Studio for Gaussian Splatting dataset and model generation."
                ]
            }
        ],
        achievements: [
            { text: "Tuition exemption for the 2025–2026 academic year", sub: ", Mines Paris PSL (Present)" },
            { text: "Tuition exemption during all academic periods", sub: ", Universidad Nacional de Colombia (Present)" },
            { text: "Recognition: Top University GPA in Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Recognition: Top STEM University GPA in Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Honor Roll 2022-2", sub: ", Universidad Nacional de Colombia (2022)" },
            { text: "Recognition: Resilience and personal improvement", sub: ", Colegio Rodrigo Arenas Betancourt (2020)" }
        ],
        volunteering: [
            {
                company: "AspirantesVirtual",
                date: "Jan 2025 – Jun 2025",
                role: "Teacher – Colombia",
                tasks: [
                    "Development of classes and tutoring in Physics and Mathematics topics for Universidad Nacional de Colombia applicants."
                ]
            },
            {
                company: "Colombian Virtual Reality Foundation",
                date: "Mar 2020 – Jun 2021",
                role: "Developer – Colombia",
                tasks: [
                    "Developed game features to raise funds and supported social initiatives aiding vulnerable communities during the COVID-19 pandemic."
                ]
            }
        ],
        hobbies: [
            { category: "Competitive Programming:", desc: "Passionate about competitive programming, interested in solving algorithmic problems and participating in programming competitions. Member of a team participating in the ICPC Unal event in July 2024." },
            { category: "Video Games:", desc: "Curious player, exploring various genres and platforms, appreciating the graphic art and narrative within video games." }
        ]
    },
    es: {
        subtitle: "Estudiante de Doble Titulación en Mines Paris PSL y Universidad Nacional de Colombia",
        nav: {
            profile: "Perfil",
            education: "Educación",
            experience: "Experiencia",
            skills: "Habilidades",
            projects: "Proyectos",
            achievements: "Logros",
            volunteering: "Voluntariado",
            hobbies: "Aficiones",
            pdf: "Versión PDF"
        },
        section: {
            profile: "Perfil",
            education: "Educación",
            employment: "Historial Laboral",
            skills: "Habilidades",
            projects: "Proyectos Universitarios",
            achievements: "Logros",
            volunteering: "Voluntariado",
            hobbies: "Aficiones"
        },
        profile: {
            text: "Estudiante de Ingeniería de Sistemas con experiencia como ingeniero de software y una fuerte pasión por las matemáticas y la informática. Actualmente cursando una doble titulación internacional entre la Universidad Nacional de Colombia y Mines Paris PSL. Siempre buscando aprender, construir herramientas significativas y ser parte de proyectos que generen impacto. ¡Adoro la IA y el Aprendizaje Automático!"
        },
        education: [
            {
                school: "Mines Paris PSL",
                date: "Ago 2025 – Jul 2027",
                degree: "Máster en Ingeniería, Matemáticas Computacionales y Aplicadas"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "Oct 2021 – Sep 2027",
                degree: "Pregrado en Ingeniería de Sistemas"
            },
            {
                school: "Universidad Técnica Checa en Praga",
                date: "Nov 2025",
                degree: "Programa ATHENS - Métodos Probabilísticos en Combinatoria"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "Abr 2025 – Ago 2025",
                role: "Ingeniero de Software – Miami, Estados Unidos (Remoto)",
                tasks: [
                    "Desarrollo y mantenimiento de herramientas para la creación, preprocesamiento y aseguramiento de calidad de datasets para modelos de visión por computadora basados en video.",
                    "Implementación de flujos de trabajo automatizados para convertir, validar y depurar anotaciones entre formatos (YOLO, COCO) para apoyar pipelines de entrenamiento.",
                    "Asistencia en el diagnóstico y resolución de problemas de integridad de datos y etiquetado de la plataforma de anotación interna.",
                    "Contribución a experimentos de entrenamiento de modelos (ej. RT-DETR) para verificar la corrección del dataset y mejorar la fiabilidad del entrenamiento."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Nov 2024 – Abr 2025",
                role: "Ingeniero de Software – Bogotá, Colombia (Remoto)",
                tasks: [
                    "Participación en el desarrollo de tres aplicaciones web: dos para automatizar el procesamiento de archivos geoespaciales en consultoría de ingeniería civil (backend), y una para gestión de proyectos y estimación de costos (full-stack).",
                    "Diseño e implementación de interfaces de usuario, funciones de automatización de costos y flujos de integración de datos con APIs externas.",
                    "Desarrollo de algoritmos para manejo de datos externos, parsers de archivos y sistemas de autenticación.",
                    "Integración y gestión de bases de datos MySQL/PostgreSQL, aplicando principios de arquitectura limpia para código escalable y mantenible."
                ]
            },
            {
                company: "Laboratorio de Sistemas Inteligentes (LISI)",
                date: "Ago 2023 – Presente",
                role: "Estudiante Investigador de Pregrado – Bogotá, Colombia",
                tasks: [
                    "Miembro activo del grupo de investigación, participando en seminarios sobre inteligencia artificial y proyectos colaborativos con estudiantes de pregrado y posgrado.",
                    "Preparación y presentación de materiales para seminarios sobre técnicas y arquitecturas de aprendizaje profundo (CNNs, VAEs, Transformers, etc.).",
                    "Contribución a la formulación de propuestas presentadas a convocatorias de Minciencias 950 y 966, integrando inteligencia artificial y realidad virtual para educación ambiental."
                ]
            },
        ],
        skills: [
            { category: "Idiomas:", list: "Español (Nativo) • Inglés (C1) • Francés (B2) • Portugués (Conversacional)" },
            { category: "Habilidades Blandas:", list: "Trabajo en equipo • Resolución de problemas • Adaptabilidad • Comunicación • Creatividad • Pensamiento crítico" },
            { category: "Programación:", list: "Python (FastAPI, Pandas, NumPy, PyTorch, TensorFlow, etc.) • Julia • Java • C/C++ • Typescript (React) • SQL (PostgreSQL, MySQL, Sqlite) • No SQL (Redis, MongoDB)" },
            { category: "Herramientas Técnicas:", list: "Git • AWS • Azure • GCP • Jira • Wolfram Mathematica • LaTeX (Overleaf) • Microsoft Office" }
        ],
        projects: [
            {
                title: "Pipeline Automatizado para Generación de Modelos Gaussian Splatting",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Desarrollo de una exposición virtual inmersiva de artefactos geológicos utilizando Gaussian splats para mejorar reflectancia, translucidez y realismo.",
                    "Construcción de un pipeline automatizado integrando AWS (S3 y Lambda), RealityScan y LightField Studio para la generación de datasets y modelos de Gaussian Splatting."
                ]
            }
        ],
        achievements: [
            { text: "Exención de matrícula para el año académico 2025–2026", sub: ", Mines Paris PSL (Presente)" },
            { text: "Exención de matrícula durante todos los periodos académicos", sub: ", Universidad Nacional de Colombia (Presente)" },
            { text: "Reconocimiento: Mejor Promedio Universitario en Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Reconocimiento: Mejor Promedio Universitario STEM en Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Cuadro de Honor 2022-2", sub: ", Universidad Nacional de Colombia (2022)" },
            { text: "Reconocimiento: Resiliencia y superación personal", sub: ", Colegio Rodrigo Arenas Betancourt (2020)" }
        ],
        volunteering: [
            {
                company: "AspirantesVirtual",
                date: "Ene 2025 – Jun 2025",
                role: "Profesor – Colombia",
                tasks: [
                    "Desarrollo de clases y tutorías en temas de Física y Matemáticas para aspirantes a la Universidad Nacional de Colombia."
                ]
            },
            {
                company: "Fundación Colombiana de Realidad Virtual",
                date: "Mar 2020 – Jun 2021",
                role: "Desarrollador – Colombia",
                tasks: [
                    "Desarrollo de características de juegos para recaudar fondos y apoyar iniciativas sociales ayudando a comunidades vulnerables durante la pandemia de COVID-19."
                ]
            }
        ],
        hobbies: [
            { category: "Programación Competitiva:", desc: "Apasionado por la programación competitiva, interesado en resolver problemas algorítmicos y participar en competiciones de programación. Miembro de un equipo participante en el evento ICPC Unal en Julio de 2024." },
            { category: "Videojuegos:", desc: "Jugador curioso, explorando varios géneros y plataformas, apreciando el arte gráfico y la narrativa dentro de los videojuegos." }
        ]
    },
    fr: {
        subtitle: "Étudiant en Double Diplôme à Mines Paris PSL et Universidad Nacional de Colombia",
        nav: {
            profile: "Profil",
            education: "Formation",
            experience: "Expérience",
            skills: "Compétences",
            projects: "Projets",
            achievements: "Réalisations",
            volunteering: "Bénévolat",
            hobbies: "Loisirs",
            pdf: "Version PDF"
        },
        section: {
            profile: "Profil",
            education: "Formation",
            employment: "Expérience Professionnelle",
            skills: "Compétences",
            projects: "Projets Universitaires",
            achievements: "Réalisations",
            volunteering: "Bénévolat",
            hobbies: "Loisirs"
        },
        profile: {
            text: "Étudiant en ingénierie informatique en double diplôme international entre l’Universidad Nacional de Colombia et l’École des Mines de Paris. Fort d’une première expérience en développement logiciel et d’un solide socle en mathématiques, je suis motivé par la création de solutions utiles et l’innovation technologique. Passionné par l’IA et le Machine Learning, je suis disponible à partir de mai 2026 pour des opportunités en France ou à l’international."
        },
        education: [
            {
                school: "Mines Paris PSL",
                date: "Août 2025 – Juil 2027",
                degree: "Master en Sciences et Ingénierie, Mathématiques Appliquées et Calcul"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "Oct 2021 – Sep 2027",
                degree: "Licence en Génie Informatique"
            },
            {
                school: "Université Technique Tchèque de Prague",
                date: "Nov 2025",
                degree: "Programme ATHENS - Méthodes Probabilistes en Combinatoire"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "Avr 2025 – Aoû 2025",
                role: "Ingénieur Logiciel – Miami, États-Unis (Télétravail)",
                tasks: [
                    "Développement et maintenance d'outils pour la création, le prétraitement et l'assurance qualité de jeux de données pour modèles de vision par ordinateur basés sur la vidéo.",
                    "Mise en œuvre de flux de travail automatisés pour convertir, valider et déboguer les annotations entre formats (YOLO, COCO) pour soutenir les pipelines d'entraînement.",
                    "Assistance au diagnostic et à la résolution des problèmes d'intégrité des données et d'étiquetage de la plateforme d'annotation interne.",
                    "Contribution aux expériences d'entraînement de modèles (ex. RT-DETR) pour vérifier la correction des données et améliorer la fiabilité de l'entraînement."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Nov 2024 – Avr 2025",
                role: "Ingénieur Logiciel – Bogotá, Colombie (Télétravail)",
                tasks: [
                    "Participation au développement de trois applications web : deux pour l'automatisation du traitement de fichiers géospatiaux dans le conseil en génie civil (backend), et une pour la gestion de projets et l'estimation des coûts (full-stack).",
                    "Conception et mise en œuvre d'interfaces utilisateur, de fonctionnalités d'automatisation des coûts et de flux d'intégration de données avec des API externes.",
                    "Développement d'algorithmes pour la gestion de données externes, d'analyseurs de fichiers et de systèmes d'authentification.",
                    "Intégration et gestion de bases de données MySQL/PostgreSQL, application des principes d'architecture propre pour un code évolutif et maintenable."
                ]
            },
            {
                company: "Laboratoire des Systèmes Intelligents (LISI)",
                date: "Août 2023 – Présent",
                role: "Chercheur Étudiant – Bogotá, Colombie",
                tasks: [
                    "Membre actif du groupe de recherche, participant à des séminaires sur l'intelligence artificielle et à des projets collaboratifs avec des étudiants de premier et deuxième cycles.",
                    "Préparation et présentation de matériels pour des séminaires sur les techniques et architectures d'apprentissage profond (CNN, VAE, Transformers, etc.).",
                    "Contribution à la formulation de propositions soumises aux appels Minciencias 950 et 966, intégrant l'intelligence artificielle et la réalité virtuelle pour l'éducation environnementale."
                ]
            }
        ],
        skills: [
            { category: "Langues :", list: "Espagnol (Natif) • Anglais (C1) • Français (B2) • Portugais (Conversationnel)" },
            { category: "Compétences Transversales :", list: "Travail d'équipe • Résolution de problèmes • Adaptabilité • Communication • Créativité • Pensée critique" },
            { category: "Programmation :", list: "Python (FastAPI, Pandas, NumPy, PyTorch, TensorFlow, etc.) • Julia • Java • C/C++ • Typescript (React) • SQL (PostgreSQL, MySQL, Sqlite) • No SQL (Redis, MongoDB)" },
            { category: "Outils Techniques :", list: "Git • AWS • Azure • GCP • Jira • Wolfram Mathematica • LaTeX (Overleaf) • Microsoft Office" }
        ],
        projects: [
            {
                title: "Pipeline Automatisé pour la Génération de Modèles Gaussian Splatting",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Développement d'une exposition virtuelle immersive d'artefacts géologiques utilisant des Gaussian splats pour améliorer la réflectance, la translucidité et le réalisme.",
                    "Construction d'un pipeline automatisé intégrant AWS (S3 et Lambda), RealityScan et LightField Studio pour la génération de jeux de données et de modèles Gaussian Splatting."
                ]
            }
        ],
        achievements: [
            { text: "Exonération des frais de scolarité 2025–2026", sub: ", Mines Paris PSL (Présent)" },
            { text: "Exonération des frais pour toutes les périodes académiques", sub: ", Universidad Nacional de Colombia (Présent)" },
            { text: "Reconnaissance : Meilleur GPA universitaire à Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Reconnaissance : Meilleur GPA STEM à Bogotá", sub: ", Premios Jóvenes a la Excelencia (2024)" },
            { text: "Tableau d'Honneur 2022-2", sub: ", Universidad Nacional de Colombia (2022)" },
            { text: "Reconnaissance : Résilience et dépassement personnel", sub: ", Colegio Rodrigo Arenas Betancourt (2020)" }
        ],
        volunteering: [
            {
                company: "AspirantesVirtual",
                date: "Jan 2025 – Juin 2025",
                role: "Enseignant – Colombie",
                tasks: [
                    "Développement de cours et tutorats en Physique et Mathématiques pour les candidats à l'Universidad Nacional de Colombia."
                ]
            },
            {
                company: "Fondation Colombienne de Réalité Virtuelle",
                date: "Mar 2020 – Juin 2021",
                role: "Développeur – Colombie",
                tasks: [
                    "Développement de fonctionnalités de jeu pour collecter des fonds et soutenir des initiatives sociales aidant les communautés vulnérables pendant la pandémie de COVID-19."
                ]
            }
        ],
        hobbies: [
            { category: "Programmation Compétitive :", desc: "Passionné par la programmation compétitive, intéressé par la résolution de problèmes algorithmiques et la participation à des concours de programmation. Membre d'une équipe participant à l'événement ICPC Unal en juillet 2024." },
            { category: "Jeux Vidéo :", desc: "Joueur curieux, explorant divers genres et plateformes, appréciant l'art graphique et la narration dans les jeux vidéo." }
        ]
    }
};

function setLanguage(lang) {
    const data = translations[lang];

    // 1. Simple text replacements
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        const keys = key.split('.');
        let value = data;
        for (const k of keys) {
            if (value && value[k]) {
                value = value[k];
            } else {
                value = null;
                break;
            }
        }
        if (value && typeof value === 'string') {
            element.innerText = value;
        }
    });

    // 2. Render Education
    const educationList = document.getElementById('education-list');
    educationList.innerHTML = '';
    data.education.forEach(edu => {
        const entry = document.createElement('div');
        entry.className = 'entry';
        entry.innerHTML = `
            <div class="entry-header">
                <span>${edu.school}</span>
                <span>${edu.date}</span>
            </div>
            <div class="entry-subheader">${edu.degree}</div>
        `;
        educationList.appendChild(entry);
    });

    // 3. Render Employment
    const employmentList = document.getElementById('employment-list');
    employmentList.innerHTML = '';
    data.employment.forEach(job => {
        const entry = document.createElement('div');
        entry.className = 'entry';
        let tasksHtml = '<ul>';
        job.tasks.forEach(task => tasksHtml += `<li>${task}</li>`);
        tasksHtml += '</ul>';
        
        entry.innerHTML = `
                <div class="entry-header">
                    <span>${job.company}</span>
                    <span>${job.date}</span>
                </div>
                <div class="entry-subheader">${job.role}</div>
            ${tasksHtml}
        `;
        employmentList.appendChild(entry);
    });

    // 4. Render Skills
    const skillsList = document.getElementById('skills-list');
    skillsList.innerHTML = '';
    data.skills.forEach(skill => {
        const li = document.createElement('li');
        li.innerHTML = `<span class="skills-category">${skill.category}</span> ${skill.list}`;
        skillsList.appendChild(li);
    });

    // 5. Render Projects
    const projectsList = document.getElementById('projects-list');
    projectsList.innerHTML = '';
    data.projects.forEach(proj => {
        const entry = document.createElement('div');
        entry.className = 'entry';
        let tasksHtml = '<ul>';
        proj.tasks.forEach(task => tasksHtml += `<li>${task}</li>`);
        tasksHtml += '</ul>';
        
        entry.innerHTML = `
                <div class="entry-header">
                <span>${proj.title}</span>
                <span>${proj.date}</span>
            </div>
            <div class="entry-subheader">${proj.subheader}</div>
            ${tasksHtml}
        `;
        projectsList.appendChild(entry);
    });

    // 6. Render Achievements
    const achievementsList = document.getElementById('achievements-list');
    achievementsList.innerHTML = '';
    data.achievements.forEach(ach => {
        const li = document.createElement('li');
        li.innerHTML = `<span class="skills-category">${ach.text}</span>${ach.sub || ''}`;
        achievementsList.appendChild(li);
    });

    // 7. Render Volunteering
    const volunteeringList = document.getElementById('volunteering-list');
    volunteeringList.innerHTML = '';
    data.volunteering.forEach(vol => {
        const entry = document.createElement('div');
        entry.className = 'entry';
        let tasksHtml = '<ul>';
        vol.tasks.forEach(task => tasksHtml += `<li>${task}</li>`);
        tasksHtml += '</ul>';
        
        entry.innerHTML = `
                <div class="entry-header">
                <span>${vol.company}</span>
                <span>${vol.date}</span>
            </div>
            <div class="entry-subheader">${vol.role}</div>
            ${tasksHtml}
        `;
        volunteeringList.appendChild(entry);
    });

    // 8. Render Hobbies
    const hobbiesList = document.getElementById('hobbies-list');
    hobbiesList.innerHTML = '';
    data.hobbies.forEach(hobby => {
        const li = document.createElement('li');
        li.innerHTML = `<span class="skills-category">${hobby.category}</span> ${hobby.desc}`;
        hobbiesList.appendChild(li);
    });

    // Update language switcher active state
    document.querySelectorAll('.language-btn').forEach(btn => {
        btn.classList.remove('active');
        if (btn.innerText.toLowerCase() === lang) {
            btn.classList.add('active');
        }
    });
    
    // Save preference
    localStorage.setItem('preferredLanguage', lang);
    document.documentElement.lang = lang;
}

// Initialize language
const savedLang = localStorage.getItem('preferredLanguage') || 'en';
setLanguage(savedLang);

// Mobile Navbar Hiding Logic
let lastScrollTop = 0;
const navbar = document.querySelector('.navbar');

window.addEventListener('scroll', function() {
    // Only apply on mobile view (768px matches CSS breakpoint)
    if (window.innerWidth <= 768) {
        let scrollTop = window.pageYOffset || document.documentElement.scrollTop;
        if (scrollTop > lastScrollTop) {
            // Scrolling down
            navbar.style.top = '-200px'; // Adjust based on navbar height
        } else {
            // Scrolling up
            navbar.style.top = '0';
        }
        lastScrollTop = scrollTop;
    } else {
        // Reset for desktop
        navbar.style.top = '0';
    }
});
