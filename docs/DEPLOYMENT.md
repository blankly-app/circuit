# Deploying Circuit Documentation

This guide explains how to deploy the Circuit documentation to various hosting platforms.

## GitHub Pages (Recommended)

GitHub Pages is the recommended hosting option for Circuit documentation. It's free, fast, and integrates seamlessly with GitHub Actions.

### Initial Setup

1. **Enable GitHub Pages**
   - Go to your repository on GitHub
   - Click **Settings** → **Pages**
   - Under **Source**, select **GitHub Actions**
   - Click **Save**

2. **Verify Workflow Permissions**
   - Go to **Settings** → **Actions** → **General**
   - Under **Workflow permissions**, ensure **Read and write permissions** is selected
   - Check **Allow GitHub Actions to create and approve pull requests**
   - Click **Save**

3. **Trigger Initial Deployment**

   Option A: Push to main branch
   ```bash
   git checkout main
   git push
   ```

   Option B: Manual trigger
   - Go to **Actions** tab
   - Select **Deploy Documentation**
   - Click **Run workflow** → **Run workflow**

4. **Access Your Documentation**

   After the workflow completes (usually 2-3 minutes):
   - Your docs will be available at: `https://<username>.github.io/<repo-name>/`
   - For example: `https://blankly-app.github.io/circuit/`

### Automatic Updates

Once set up, documentation automatically rebuilds and deploys whenever you:
- Push to the `main` branch
- Merge a pull request to `main`

### Custom Domain (Optional)

To use a custom domain like `docs.circuit.dev`:

1. **Add CNAME file**
   ```bash
   echo "docs.circuit.dev" > docs/book/CNAME
   ```

2. **Configure DNS**
   - Add a CNAME record pointing to `<username>.github.io`
   - Example: `docs.circuit.dev` → `blankly-app.github.io`

3. **Update GitHub Settings**
   - Go to **Settings** → **Pages**
   - Enter your custom domain
   - Check **Enforce HTTPS**

### Troubleshooting

**Workflow fails with permission errors:**
- Check workflow permissions in Settings → Actions → General
- Ensure **Read and write permissions** is enabled

**Pages not updating:**
- Check the Actions tab for failed workflows
- Ensure the workflow completed successfully
- Clear browser cache (Ctrl+Shift+R)

**404 errors:**
- Verify GitHub Pages is enabled in Settings → Pages
- Check that the workflow deployed successfully
- Wait a few minutes for DNS/CDN propagation

## Netlify

Netlify offers additional features like deploy previews and form handling.

### Setup

1. **Sign up at [netlify.com](https://netlify.com)**

2. **Connect Repository**
   - Click **New site from Git**
   - Choose GitHub and authorize
   - Select your Circuit repository

3. **Configure Build Settings**
   - Build command: `cd docs && ./build.sh`
   - Publish directory: `docs/book`
   - Click **Deploy site**

4. **Custom Domain (Optional)**
   - Go to **Domain settings**
   - Add your custom domain
   - Update DNS as instructed

### Deploy Previews

Netlify automatically creates preview deployments for pull requests:
- Each PR gets a unique URL
- Test changes before merging
- Preview link appears in PR comments

### Environment Variables

If needed, add environment variables in **Site settings** → **Environment variables**.

## Vercel

Vercel offers edge caching and excellent performance.

### Setup

1. **Sign up at [vercel.com](https://vercel.com)**

2. **Import Repository**
   - Click **New Project**
   - Import your Circuit repository

3. **Configure Project**
   - Framework Preset: **Other**
   - Build Command: `cd docs && ./build.sh`
   - Output Directory: `docs/book`
   - Click **Deploy**

4. **Custom Domain**
   - Go to **Settings** → **Domains**
   - Add your domain
   - Update DNS as instructed

## Self-Hosted

Host the documentation on your own server.

### Build Documentation

```bash
cd docs
./build.sh
```

### Deploy to Server

**Using rsync:**
```bash
rsync -avz book/ user@server:/var/www/docs/
```

**Using Docker:**
```dockerfile
FROM nginx:alpine
COPY docs/book /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

Build and run:
```bash
docker build -t circuit-docs .
docker run -d -p 80:80 circuit-docs
```

### Nginx Configuration

```nginx
server {
    listen 80;
    server_name docs.circuit.dev;

    root /var/www/circuit/docs;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }

    # Enable gzip
    gzip on;
    gzip_types text/html text/css application/javascript;

    # Cache static assets
    location ~* \.(css|js|png|jpg|jpeg|gif|ico|svg)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## Cloudflare Pages

Cloudflare Pages offers global CDN and zero-config deployments.

### Setup

1. **Sign up at [pages.cloudflare.com](https://pages.cloudflare.com)**

2. **Connect Repository**
   - Click **Create a project**
   - Connect GitHub account
   - Select Circuit repository

3. **Configure Build**
   - Build command: `cd docs && ./build.sh`
   - Build output directory: `docs/book`
   - Click **Save and Deploy**

## AWS S3 + CloudFront

For enterprise deployments with AWS infrastructure.

### Build and Upload

```bash
# Build documentation
cd docs && ./build.sh

# Upload to S3
aws s3 sync book/ s3://circuit-docs-bucket/ --delete

# Invalidate CloudFront cache
aws cloudfront create-invalidation --distribution-id YOUR_DIST_ID --paths "/*"
```

### Automation Script

```bash
#!/bin/bash
set -e

echo "Building documentation..."
cd docs
./build.sh

echo "Uploading to S3..."
aws s3 sync book/ s3://circuit-docs-bucket/ \
    --delete \
    --cache-control "public, max-age=3600"

echo "Invalidating CloudFront cache..."
aws cloudfront create-invalidation \
    --distribution-id YOUR_DIST_ID \
    --paths "/*"

echo "Documentation deployed successfully!"
```

## Comparison

| Platform | Pros | Cons | Cost |
|----------|------|------|------|
| **GitHub Pages** | Free, Simple, Automated | Limited features | Free |
| **Netlify** | Deploy previews, Forms, Functions | Build minutes limited | Free tier available |
| **Vercel** | Fast edge network, Analytics | Build time limits | Free tier available |
| **Cloudflare Pages** | Global CDN, Unlimited bandwidth | Learning curve | Free |
| **Self-hosted** | Full control, No limits | Maintenance required | Server costs |

## Best Practices

1. **Always test locally first**
   ```bash
   cd docs
   ./build.sh
   mdbook serve --open
   ```

2. **Use versioning for releases**
   - Tag releases: `v1.0.0`, `v1.1.0`
   - Create version-specific docs
   - Maintain changelog

3. **Monitor deployment**
   - Check workflow status
   - Test deployed site
   - Verify all links work

4. **Set up analytics** (optional)
   - Google Analytics
   - Plausible Analytics
   - Simple Analytics

## Continuous Integration

The GitHub Actions workflow (`.github/workflows/deploy-docs.yml`) handles:

- ✅ Building cargo documentation
- ✅ Building mdBook documentation
- ✅ Integrating cargo docs with mdBook
- ✅ Deploying to GitHub Pages
- ✅ Caching for faster builds

### Workflow Triggers

- Push to `main` branch
- Manual trigger via Actions tab
- Scheduled builds (optional)

## Questions?

- Check the [GitHub Actions Documentation](https://docs.github.com/en/actions)
- See [mdBook Documentation](https://rust-lang.github.io/mdBook/)
- Open an issue on GitHub
