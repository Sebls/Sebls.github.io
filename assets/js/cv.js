const translations = {
    en: {
        subtitle: "Computer Engineering student pursuing a Master’s degree in Engineering and Applied Mathematics at Mines Paris–PSL",
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
            text: "Computer Engineering student pursuing a Master’s degree in Engineering and Applied Mathematics at Mines Paris–PSL. Experienced in building backend services and AI models, with strong computer science fundamentals (data structures, algorithms, system design, and machine learning). Interested in performance, scalability, and media-oriented workflows."
        },
        education: [
            {
                school: "École des Mines de Paris - PSL",
                date: "2025 – Expected 2027",
                degree: "M.Sc. in Engineering and Applied Mathematics"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "2021 – Expected 2027",
                degree: "B.Sc. in Computer Engineering (GPA: 4.74/5.0)"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "Sep 2025 – Dec 2025",
                role: "Software Engineer – Miami, United States (Remote)",
                tasks: [
                    "Improved dataset preparation and creation workflows for computer vision models, achieving 60–70% faster processing times.",
                    "Implemented automated annotation conversion, validation, and debugging across YOLO and COCO, reducing manual QA from 15–18 steps to 3–4 steps.",
                    "Designed and developed an end-to-end deepfake detection application, integrating model inference, backend services, and user-facing workflows for video and audio analysis.",
                    "Deployed and maintained production inference endpoints, including autoscaling/scale-to-zero to optimize performance and infrastructure costs.",
                    "Contributed to training and evaluation of audio classification models, increasing accuracy from 83% to 91%."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Mar 2024 – Sep 2025",
                role: "Software Engineer – Bogotá, Colombia (Remote)",
                tasks: [
                    "Developed backend pipelines for automated geospatial file processing in civil engineering projects, reducing processing time from ~1 week to ~3 hours.",
                    "Designed and implemented automation for report generation and live business statistics, improving delivery speed by ~80% across client workflows.",
                    "Built and maintained backend systems integrating external APIs and MySQL/PostgreSQL databases using clean architecture principles."
                ]
            },
            {
                company: "Intelligent Systems Lab (LISI)",
                date: "Aug 2023 – Aug 2025",
                role: "Research Assistant (ML) – Bogotá, Colombia",
                tasks: [
                    "Collaborated on applied ML/DL research and internal projects with undergraduate and graduate researchers, with a focus on computer vision (detection, classification, recognition).",
                    "Led 10+ deep learning seminars (theory + hands-on labs) for 30–40 undergraduate/master’s students, covering neural networks and core ML techniques."
                ]
            }
        ],
        skills: [
            { category: "Soft Skills:", list: "Teamwork • Problem-solving • Adaptability • Communication • Creativity • Critical thinking" },
            { category: "Programming:", list: "Python (FastAPI, PyTorch, TensorFlow, OpenCV, etc.) • C/C++ • Julia • SQL (PostgreSQL, MySQL, Sqlite) • NoSQL (Redis, MongoDB)" },
            { category: "Technical Tools:", list: "Git • Docker • Amazon Web Service • Azure • Google Cloud Platform" },
            { category: "Languages:", list: "Spanish (Native) • English (C1) • French (B2) • Portuguese (Conversational)" }
        ],
        projects: [
            {
                title: "Automated Pipeline for Gaussian Splatting Model Generation",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Developed an immersive virtual exhibition of geological artifacts using Gaussian Splatting to improve realism and visual quality.",
                    "Built a ~90% automated pipeline for Gaussian Splatting dataset and model generation, reducing preparation time from ~5 hours to ~30 minutes (excluding training runtime)."
                ]
            }
        ],
        achievements: [
            { text: "Tuition exemption for the 2025–2026 academic year", sub: ", Mines Paris PSL (Present)" },
            { text: "Tuition exemption during all academic periods; Honor Roll recipient (2022)", sub: ", Universidad Nacional de Colombia (Present)" },
            { text: "Top 1% GPA among university students in Bogotá (≈40 universities)", sub: `, <a href="https://www.agenciaatenea.gov.co/premios-jovenes-la-e-2024" target="_blank" rel="noopener">Youth for Excellence</a> (2024)` },
            { text: "ICPC UNAL contest", sub: ", Competitive Programming Contest participant (2024)" }
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
        hobbies: []
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
            text: "Estudiante de Ingeniería de Sistemas en doble titulación internacional entre la Universidad Nacional de Colombia y Mines Paris PSL. Experiencia construyendo servicios backend y pipelines de datos/ML, con sólidos fundamentos (estructuras de datos, algoritmos y diseño de sistemas). Interesado en performance, escalabilidad y flujos de trabajo orientados a medios. Disponible para una práctica de verano desde mayo de 2026 (Francia o en el exterior); con alta motivación por aprender rápido y aportar en entornos colaborativos."
        },
        education: [
            {
                school: "École des Mines de Paris - PSL",
                date: "2025 – Previsto 2027",
                degree: "M.Sc. en Ingeniería y Matemáticas Aplicadas"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "2021 – Previsto 2027",
                degree: "Pregrado en Ingeniería de Sistemas (Promedio: 4.74/5.0)"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "Abr 2025 – Ago 2025",
                role: "Ingeniero de Software – Miami, Estados Unidos (Remoto)",
                tasks: [
                    "Mejoré los flujos de preparación y creación de datasets para modelos de ML de video y audio, logrando 70–80% de mejora en tiempos de procesamiento.",
                    "Implementé conversión, validación y depuración automática de anotaciones entre YOLO y COCO, reduciendo QA manual de 15–18 pasos a 3–4 pasos.",
                    "Desplegué y mantuve endpoints de inferencia en producción manejando miles de solicitudes diarias, incluyendo autoscaling/scale-to-zero para optimizar performance y costos.",
                    "Contribuí al entrenamiento y evaluación de modelos de clasificación de audio, aumentando la precisión de 83% a 91%."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Nov 2024 – Abr 2025",
                role: "Ingeniero de Software – Bogotá, Colombia (Remoto)",
                tasks: [
                    "Desarrollé pipelines backend para automatizar el procesamiento de archivos geoespaciales en proyectos de ingeniería civil, reduciendo el tiempo de ~1 semana a ~3 horas.",
                    "Diseñé e implementé automatización para generación de reportes y estadísticas de negocio en tiempo real, mejorando la velocidad de entrega en ~80% en flujos de clientes.",
                    "Construí y mantuve sistemas backend integrando APIs externas y bases de datos MySQL/PostgreSQL usando principios de arquitectura limpia."
                ]
            },
            {
                company: "Laboratorio de Sistemas Inteligentes (LISI)",
                date: "Ago 2023 – Ago 2025",
                role: "Asistente de Investigación (ML) – Bogotá, Colombia",
                tasks: [
                    "Colaboré en investigación aplicada en ML y proyectos internos con investigadores de pregrado y posgrado.",
                    "Lideré 10+ seminarios de deep learning (teoría + laboratorios) para 30–40 estudiantes de pregrado/maestría, cubriendo redes neuronales y técnicas base."
                ]
            },
        ],
        skills: [
            { category: "Habilidades Blandas:", list: "Trabajo en equipo • Resolución de problemas • Adaptabilidad • Comunicación • Creatividad • Pensamiento crítico" },
            { category: "Fundamentos CS:", list: "Estructuras de datos • Algoritmos • Fundamentos de diseño de sistemas" },
            { category: "Programación:", list: "Python (FastAPI, Pandas, NumPy, PyTorch, TensorFlow, etc.) • C/C++ • Julia • Java • Typescript (React) • SQL (PostgreSQL, MySQL, Sqlite) • NoSQL (Redis, MongoDB)" },
            { category: "Herramientas Técnicas:", list: "Git • Docker • Amazon Web Services • Azure • Google Cloud Platform" },
            { category: "Idiomas:", list: "Español (Nativo) • Inglés (C1) • Francés (B2) • Portugués (Conversacional)" }
        ],
        projects: [
            {
                title: "Pipeline Automatizado para Generación de Modelos Gaussian Splatting",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Desarrollé una exposición virtual inmersiva de artefactos geológicos usando Gaussian Splatting para mejorar el realismo y la calidad visual.",
                    "Construí un pipeline ~90% automatizado para generar datasets y modelos de Gaussian Splatting, reduciendo el tiempo de preparación de ~5 horas a ~30 minutos (sin contar tiempo de entrenamiento)."
                ]
            }
        ],
        achievements: [
            { text: "Exención de matrícula para el año académico 2025–2026", sub: ", Mines Paris PSL (Presente)" },
            { text: "Exención de matrícula durante todos los periodos; Cuadro de Honor (2022)", sub: ", Universidad Nacional de Colombia (Presente)" },
            { text: "Top 1% del GPA entre estudiantes universitarios en Bogotá (~40 universidades)", sub: ", Youth for Excellence (2024)" },
            { text: "Concurso ICPC UNAL", sub: ", participante en programación competitiva (2024)" }
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
        hobbies: []
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
            text: "Étudiant en ingénierie informatique poursuivant un double diplôme international entre l’Universidad Nacional de Colombia et Mines Paris PSL. Avec une expérience en développement logiciel et une solide base en mathématiques, je suis motivé par la création de solutions utiles et l’innovation technologique. Passionné par l’IA et l’apprentissage automatique, je suis disponible à partir de mai 2026 pour des opportunités en France ou à l’international."
        },
        education: [
            {
                school: "École des Mines de Paris - PSL",
                date: "2025 – Prévu 2027",
                degree: "M.Sc. en ingénierie et mathématiques appliquées"
            },
            {
                school: "Universidad Nacional de Colombia",
                date: "2021 – Prévu 2027",
                degree: "Licence en ingénierie informatique (GPA : 4,74/5,0)"
            }
        ],
        employment: [
            {
                company: "Engin AI",
                date: "avr. 2025 – août 2025",
                role: "Ingénieur logiciel – Miami, États-Unis (à distance)",
                tasks: [
                    "Amélioration des flux de travail de création et de préparation de jeux de données pour des modèles d’IA audio et vidéo, avec 70–80% de gains de performance.",
                    "Mise en place de conversions, validations et débogages automatisés d’annotations entre YOLO et COCO, réduisant le contrôle qualité manuel de 15–18 étapes à 3–4.",
                    "Déploiement de services d’inférence en production traitant des milliers de requêtes quotidiennes, incluant une mise à l’échelle automatique jusqu’à zéro pour réduire les coûts d’infrastructure.",
                    "Contribution à l’entraînement et à l’évaluation de modèles de classification audio, augmentant la précision de 83% à 91%."
                ]
            },
            {
                company: "Dataconstructors AI",
                date: "Nov 2024 – Avr 2025",
                role: "Ingénieur logiciel – Bogotá, Colombie (à distance)",
                tasks: [
                    "Développement de pipelines back-end pour automatiser le traitement de fichiers géospatiaux pour des projets de génie civil, réduisant le temps de traitement de ~1 semaine à ~3 heures.",
                    "Conception et implémentation d’automatisations pour la génération de rapports et des statistiques métier en temps réel, améliorant la vitesse de livraison d’environ ~80% dans les processus clients.",
                    "Développement et maintenance de systèmes full-stack et back-end intégrant des API externes et des bases MySQL/PostgreSQL, en appliquant les principes de Clean Architecture."
                ]
            },
            {
                company: "Laboratoire des Systèmes Intelligents (LISI)",
                date: "Août 2023 – Août 2025",
                role: "Assistant de recherche (IA) – Bogotá, Colombie",
                tasks: [
                    "Collaboration sur des travaux de recherche appliquée en IA et des projets internes avec des chercheurs de licence et master.",
                    "Animation de 10+ séminaires en apprentissage profond (théorie + TP) pour 30–40 étudiants de licence/master, couvrant les réseaux de neurones et les techniques fondamentales d’IA."
                ]
            }
        ],
        skills: [
            { category: "Compétences Transversales :", list: "Travail d'équipe • Résolution de problèmes • Adaptabilité • Communication • Créativité • Pensée critique" },
            { category: "Programmation :", list: "Python (FastAPI, Pandas, NumPy, PyTorch, TensorFlow, etc.) • C/C++ • Julia • Java • TypeScript (React) • SQL (PostgreSQL, MySQL, Sqlite) • NoSQL (Redis, MongoDB)" },
            { category: "Outils techniques :", list: "Git • Docker • Amazon Web Services • Azure • Google Cloud Platform" },
            { category: "Langues :", list: "Espagnol (natif) • Anglais (C1) • Français (B2) • Portugais (conversationnel)" }
        ],
        projects: [
            {
                title: "Pipeline Automatisé pour la Génération de Modèles Gaussian Splatting",
                date: "Sep 2025 – Nov 2025",
                subheader: "Mines Paris PSL - MOVIE: Ingénierie des Mondes Virtuels",
                tasks: [
                    "Développement d’une exposition virtuelle immersive d’artefacts géologiques avec le Gaussian Splatting pour améliorer le réalisme et la qualité visuelle.",
                    "Conception d’un pipeline automatisé à ~90% pour générer des jeux de données et des modèles de Gaussian Splatting, réduisant le temps de préparation de ~5 heures à ~30 minutes (hors temps d’entraînement)."
                ]
            }
        ],
        achievements: [
            { text: "Exonération des frais de scolarité pour l’année académique 2025–2026", sub: ", Mines Paris PSL (Présent)" },
            { text: "Exonération des frais sur toutes les périodes académiques ; récipiendaire du tableau d’honneur (2022)", sub: ", Universidad Nacional de Colombia (Présent)" },
            { text: "Top 1% du GPA parmi les étudiants universitaires à Bogotá (~40 universités)", sub: ", Youth for Excellence (2024)" },
            { text: "Concours ICPC UNAL", sub: ", participant au concours de programmation compétitive (2024)" }
        ],
        volunteering: [
            {
                company: "AspirantesVirtual",
                date: "janv. 2025 – juin 2025",
                role: "Enseignant – Colombie",
                tasks: [
                    "Développement de cours et tutorats en Physique et Mathématiques pour les candidats à l’Universidad Nacional de Colombia."
                ]
            },
            {
                company: "Fondation Colombienne de Réalité Virtuelle",
                date: "mars 2020 – juin 2021",
                role: "Développeur – Colombie",
                tasks: [
                    "Développement de fonctionnalités de jeu pour collecter des fonds et soutenir des initiatives sociales aidant les communautés vulnérables pendant la pandémie de COVID-19."
                ]
            }
        ],
        hobbies: []
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
    (Array.isArray(data.volunteering) ? data.volunteering : []).forEach(vol => {
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
    (Array.isArray(data.hobbies) ? data.hobbies : []).forEach(hobby => {
        const li = document.createElement('li');
        li.innerHTML = `<span class="skills-category">${hobby.category}</span> ${hobby.desc}`;
        hobbiesList.appendChild(li);
    });

    // 9. Hide/Show optional sections (if empty)
    function toggleSection(sectionId, hasData) {
        const section = document.getElementById(sectionId);
        const link = document.querySelector(`.cv-sidebar a[href="#${sectionId}"]`);
        if (section) section.style.display = hasData ? '' : 'none';
        if (link) link.style.display = hasData ? '' : 'none';
    }

    toggleSection('volunteering', Array.isArray(data.volunteering) && data.volunteering.length > 0);
    toggleSection('hobbies', Array.isArray(data.hobbies) && data.hobbies.length > 0);

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
