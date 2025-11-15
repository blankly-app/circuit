// Custom JavaScript for Circuit Documentation

(function() {
    'use strict';

    // Add copy button to code blocks
    function addCopyButtons() {
        const codeBlocks = document.querySelectorAll('pre > code');

        codeBlocks.forEach(function(codeBlock) {
            const button = document.createElement('button');
            button.className = 'copy-button';
            button.textContent = 'Copy';
            button.style.cssText = `
                position: absolute;
                top: 5px;
                right: 5px;
                padding: 4px 8px;
                font-size: 12px;
                background: var(--circuit-primary, #4299e1);
                color: white;
                border: none;
                border-radius: 3px;
                cursor: pointer;
                opacity: 0.7;
            `;

            button.addEventListener('mouseenter', function() {
                this.style.opacity = '1';
            });

            button.addEventListener('mouseleave', function() {
                this.style.opacity = '0.7';
            });

            button.addEventListener('click', function() {
                const code = codeBlock.textContent;
                navigator.clipboard.writeText(code).then(function() {
                    button.textContent = 'Copied!';
                    setTimeout(function() {
                        button.textContent = 'Copy';
                    }, 2000);
                });
            });

            const pre = codeBlock.parentElement;
            pre.style.position = 'relative';
            pre.appendChild(button);
        });
    }

    // Add anchor links to headers
    function addAnchorLinks() {
        const headers = document.querySelectorAll('h2, h3, h4');

        headers.forEach(function(header) {
            if (header.id) {
                const link = document.createElement('a');
                link.className = 'header-anchor';
                link.href = '#' + header.id;
                link.textContent = ' #';
                link.style.cssText = `
                    color: var(--circuit-primary, #4299e1);
                    text-decoration: none;
                    opacity: 0;
                    transition: opacity 0.2s;
                    margin-left: 10px;
                    font-weight: normal;
                `;

                header.appendChild(link);

                header.addEventListener('mouseenter', function() {
                    link.style.opacity = '0.6';
                });

                header.addEventListener('mouseleave', function() {
                    link.style.opacity = '0';
                });

                link.addEventListener('mouseenter', function() {
                    this.style.opacity = '1';
                });
            }
        });
    }

    // Enhance external links
    function enhanceExternalLinks() {
        const links = document.querySelectorAll('a[href^="http"]');

        links.forEach(function(link) {
            if (!link.hostname.includes(window.location.hostname)) {
                link.setAttribute('target', '_blank');
                link.setAttribute('rel', 'noopener noreferrer');
            }
        });
    }

    // Add language labels to code blocks
    function addLanguageLabels() {
        const codeBlocks = document.querySelectorAll('pre > code[class*="language-"]');

        codeBlocks.forEach(function(codeBlock) {
            const className = codeBlock.className;
            const match = className.match(/language-(\w+)/);

            if (match) {
                const language = match[1];
                const label = document.createElement('span');
                label.textContent = language;
                label.style.cssText = `
                    position: absolute;
                    top: 5px;
                    left: 10px;
                    padding: 2px 8px;
                    font-size: 11px;
                    font-weight: bold;
                    text-transform: uppercase;
                    background: rgba(0, 0, 0, 0.2);
                    color: #aaa;
                    border-radius: 3px;
                `;

                const pre = codeBlock.parentElement;
                pre.appendChild(label);
            }
        });
    }

    // Initialize when DOM is ready
    function init() {
        addCopyButtons();
        addAnchorLinks();
        enhanceExternalLinks();
        addLanguageLabels();
    }

    // Run on page load
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

    // Re-run on page changes (for mdBook's page navigation)
    window.addEventListener('load', init);
})();
