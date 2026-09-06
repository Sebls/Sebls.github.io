/**
 * cv.js
 * Dynamic CV rendering and multi-language support.
 * Data source: assets/js/cv-data.js (synchronized from sebls_cv submodule)
 */

(function () {
    // Default fallback translations if cvTranslations is not yet loaded
    const translations = (typeof window !== 'undefined' && window.cvTranslations) ? window.cvTranslations : {};

    function setLanguage(lang) {
        const data = translations[lang] || translations['en'];
        if (!data) return;

        // 1. Text and HTML replacements for data-i18n
        document.querySelectorAll('[data-i18n]').forEach(element => {
            const key = element.getAttribute('data-i18n');
            const keys = key.split('.');
            let value = data;
            for (const k of keys) {
                if (value && value[k] !== undefined) {
                    value = value[k];
                } else {
                    value = null;
                    break;
                }
            }
            if (value && typeof value === 'string') {
                if (key === 'profile.text' || key.startsWith('section.')) {
                    element.innerHTML = value;
                } else {
                    element.textContent = value;
                }
            }
        });

        // 2. Update PDF Download Link
        const pdfLink = document.getElementById('cv-pdf-link');
        if (pdfLink) {
            const pdfPath = data.pdf_path || (lang === 'fr' ? 'assets/pdf/cv_fr.pdf' : 'assets/pdf/cv_en.pdf');
            pdfLink.setAttribute('href', pdfPath);
        }

        // 3. Update Subtitle
        const subtitleEl = document.querySelector('.cv-header .subtitle');
        if (subtitleEl && data.subtitle) {
            subtitleEl.innerHTML = data.subtitle;
        }

        // 4. Update Personal Contact Info if available
        if (data.personal) {
            const emailEl = document.querySelector('.cv-contact-info a[href^="mailto:"]');
            if (emailEl && data.personal.email) {
                emailEl.href = `mailto:${data.personal.email}`;
                emailEl.innerHTML = `<i class="fas fa-envelope"></i> ${data.personal.email}`;
            }
            const phoneEl = document.querySelector('.cv-contact-info a[href^="tel:"]');
            if (phoneEl && data.personal.phone) {
                phoneEl.href = `tel:${data.personal.phone.replace(/\\s+/g, '')}`;
                phoneEl.innerHTML = `<i class="fas fa-phone"></i> ${data.personal.phone}`;
            }
        }

        // 5. Render Education
        const educationList = document.getElementById('education-list');
        if (educationList && Array.isArray(data.education)) {
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
        }

        // 6. Render Employment / Experience
        const employmentList = document.getElementById('employment-list');
        if (employmentList && Array.isArray(data.employment)) {
            employmentList.innerHTML = '';
            data.employment.forEach(job => {
                const entry = document.createElement('div');
                entry.className = 'entry';
                const tasksHtml = (Array.isArray(job.tasks) && job.tasks.length > 0)
                    ? `<ul>${job.tasks.map(task => `<li>${task}</li>`).join('')}</ul>`
                    : '';
                const roleDisplay = job.role_display || job.role || '';

                entry.innerHTML = `
                    <div class="entry-header">
                        <span>${job.company}</span>
                        <span>${job.date}</span>
                    </div>
                    <div class="entry-subheader">${roleDisplay}</div>
                    ${tasksHtml}
                `;
                employmentList.appendChild(entry);
            });
        }

        // 7. Render Skills
        const skillsList = document.getElementById('skills-list');
        if (skillsList && Array.isArray(data.skills)) {
            skillsList.innerHTML = '';
            data.skills.forEach(skill => {
                const li = document.createElement('li');
                li.innerHTML = `<span class="skills-category">${skill.category}</span> ${skill.list}`;
                skillsList.appendChild(li);
            });
        }

        // 8. Render Projects
        const projectsList = document.getElementById('projects-list');
        if (projectsList && Array.isArray(data.projects)) {
            projectsList.innerHTML = '';
            data.projects.forEach(proj => {
                const entry = document.createElement('div');
                entry.className = 'entry';
                const titleHtml = proj.url
                    ? `<a href="${proj.url}" target="_blank" rel="noopener noreferrer">${proj.title} <i class="fas fa-external-link-alt" style="font-size:0.8em"></i></a>`
                    : proj.title;
                const tasksHtml = (Array.isArray(proj.tasks) && proj.tasks.length > 0)
                    ? `<ul>${proj.tasks.map(task => `<li>${task}</li>`).join('')}</ul>`
                    : '';
                const subheader = proj.subheader || proj.role || '';

                entry.innerHTML = `
                    <div class="entry-header">
                        <span>${titleHtml}</span>
                        <span>${proj.date}</span>
                    </div>
                    ${subheader ? `<div class="entry-subheader">${subheader}</div>` : ''}
                    ${tasksHtml}
                `;
                projectsList.appendChild(entry);
            });
        }

        // 9. Render Achievements
        const achievementsList = document.getElementById('achievements-list');
        if (achievementsList && Array.isArray(data.achievements)) {
            achievementsList.innerHTML = '';
            data.achievements.forEach(ach => {
                const li = document.createElement('li');
                li.innerHTML = `<span class="skills-category">${ach.text}</span>${ach.sub || ''}`;
                achievementsList.appendChild(li);
            });
        }

        // 10. Render Volunteering
        const volunteeringList = document.getElementById('volunteering-list');
        if (volunteeringList && Array.isArray(data.volunteering)) {
            volunteeringList.innerHTML = '';
            data.volunteering.forEach(vol => {
                const entry = document.createElement('div');
                entry.className = 'entry';
                const tasksHtml = (Array.isArray(vol.tasks) && vol.tasks.length > 0)
                    ? `<ul>${vol.tasks.map(task => `<li>${task}</li>`).join('')}</ul>`
                    : '';

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
        }

        // 11. Render Hobbies
        const hobbiesList = document.getElementById('hobbies-list');
        if (hobbiesList && Array.isArray(data.hobbies)) {
            hobbiesList.innerHTML = '';
            data.hobbies.forEach(hobby => {
                const li = document.createElement('li');
                li.innerHTML = `<span class="skills-category">${hobby.category}</span> ${hobby.desc}`;
                hobbiesList.appendChild(li);
            });
        }

        // 12. Hide / Show optional sections if empty
        function toggleSection(sectionId, hasData) {
            const section = document.getElementById(sectionId);
            const link = document.querySelector(`.cv-sidebar a[href="#${sectionId}"]`);
            if (section) section.style.display = hasData ? '' : 'none';
            if (link) link.style.display = hasData ? '' : 'none';
        }

        toggleSection('volunteering', Array.isArray(data.volunteering) && data.volunteering.length > 0);
        toggleSection('achievements', Array.isArray(data.achievements) && data.achievements.length > 0);
        toggleSection('hobbies', Array.isArray(data.hobbies) && data.hobbies.length > 0);

        // 13. Update Language Switcher UI
        document.querySelectorAll('.language-btn').forEach(btn => {
            btn.classList.remove('active');
            if (btn.innerText.toLowerCase() === lang) {
                btn.classList.add('active');
            }
        });

        // 14. Save Preference
        try {
            localStorage.setItem('preferredLanguage', lang);
        } catch (e) {}
        document.documentElement.lang = lang;
    }

    // Expose setLanguage globally for onclick handlers
    window.setLanguage = setLanguage;

    // Initialize on DOM ready or immediate
    function init() {
        const savedLang = (function () {
            try {
                return localStorage.getItem('preferredLanguage');
            } catch (e) {
                return null;
            }
        })() || 'en';
        setLanguage(savedLang);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

    // Mobile Navbar hide-on-scroll logic
    let lastScrollTop = 0;
    const navbar = document.querySelector('.navbar');
    if (navbar) {
        window.addEventListener('scroll', function () {
            if (window.innerWidth <= 768) {
                const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
                if (scrollTop > lastScrollTop && scrollTop > 60) {
                    navbar.style.top = '-200px';
                } else {
                    navbar.style.top = '0';
                }
                lastScrollTop = scrollTop;
            } else {
                navbar.style.top = '0';
            }
        }, { passive: true });
    }
})();
