# Solid JS integration

## Authentication.
This example shows you how you can easily integrate Axiums HTTP-cookie authentication with your SolidJS frontend. After the authentication process has been succeeded Axium will ask the browser to save a cookie, that can be used to authenticate users.

In these examples I use the following .env file for my SolidJS project:
```
VITE_API_URL=http://127.0.0.1:8000
```


### Login page
```typescript
import { createSignal } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { Button, Card, Col, Form, Row } from "solid-bootstrap";

import { FiGithub } from "solid-icons/fi";

const Page = () => {
  // Form state
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [totp, setTotp] = createSignal("");
  const [error, setError] = createSignal("");
  const [loading, setLoading] = createSignal(false);

  const navigate = useNavigate();

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError("");
    setLoading(true);
  
    const payload: Record<string, string> = {
      email: email(),
      password: password(),
    };
    if (totp()) payload.totp = totp();
  
    try {
      const response = await fetch("/login", {  // Replace with actual API URL
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
        credentials: "include",
      });
  
      const data = await response.json();
  
      if (!response.ok) {
        throw new Error(data.error || data.message || "Login failed.");
      }
  
      if (data.success === true || response.status === 200) {
        navigate("/account/dashboard");
      } else {
        throw new Error("Login failed.");
      }
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("Login failed.");
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="auth-page">
      <Col xl={12}>
        <Card class="auth-card">
          <Card.Body class="auth-card-body">
            <Row class="g-0">
              <Col md={5} class="auth-form-section">
                <div class="auth-form-wrapper">
                  <div class="logo-box">
                    <a href="/" class="logo-link">
                      <img src="/logo.png" alt="Logo" class="logo" />
                    </a>
                  </div>

                  <h6 class="auth-title">Welcome back!</h6>
                  <p class="auth-description">Enter your credentials to access your account.</p>

                  {error() && <div class="alert alert-danger">{error()}</div>}

                  <Form class="authentication-form" onSubmit={handleSubmit}>
                    <Form.Group class="form-group">
                      <Form.Label>Email</Form.Label>
                      <Form.Control
                        type="email"
                        placeholder="Enter your email"
                        required
                        value={email()}
                        onInput={(e) => setEmail(e.currentTarget.value)}
                      />
                    </Form.Group>

                    <Form.Group class="form-group">
                      <Form.Label>Password</Form.Label>
                      <Form.Control
                        type="password"
                        placeholder="Enter your password"
                        required
                        value={password()}
                        onInput={(e) => setPassword(e.currentTarget.value)}
                      />
                    </Form.Group>

                    <Form.Group class="form-group">
                      <Form.Label>TOTP (Optional)</Form.Label>
                      <Form.Control
                        type="text"
                        placeholder="Enter your TOTP"
                        value={totp()}
                        onInput={(e) => setTotp(e.currentTarget.value)}
                      />
                    </Form.Group>

                    <div class="submit-btn-wrapper">
                      <Button variant="primary" type="submit" disabled={loading()}>
                        {loading() ? "Logging in..." : "Log In"}
                      </Button>
                    </div>
                  </Form>

                  <div class="divider">
                    <span class="divider-text">OR</span>
                  </div>

                  <Row>
                    <Col xs={12} class="text-center">
                      <Button variant="white" class="github-btn">
                        <FiGithub size={20} class="icon" />
                        Github
                      </Button>
                    </Col>
                  </Row>
                </div>
              </Col>

              <Col md={5} class="image-section">
                <div class="image-wrapper">
                  <img src="/login-image.jpg" alt="Login" class="login-image" />
                </div>
              </Col>
            </Row>
          </Card.Body>
        </Card>
      </Col>
    </div>
  );
};

export default Page;
```

### Accessing the API
This is a snippet from a component on my own site. You might have to tweak it a bit.
The most important part is that SolidJS will try to connect to the API, and use the response to paint the webpage.

```typescript
import { createSignal, onMount } from "solid-js";
import AccountLayout from "@/layouts/AccountLayout";
import PageMeta from "@/components/PageMeta";
import Projects from "@/views/account/dashboard/sections/Projects";
import Statistics from "@/views/account/dashboard/sections/Statistics";
import Tasks from "@/views/account/dashboard/sections/Tasks";

const Page = () => {
  const [user, setUser] = createSignal<{ username: string } | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  onMount(async () => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_URL}/users/current`, {
        credentials: "include", // <-- Important for cookie-based auth!
      });

      if (!response.ok) {
        throw new Error("Failed to fetch user.");
      }

      const data = await response.json();
      setUser(data);
    } catch (err) {
      setError("Could not fetch user data.");
      console.error(err);
    }
  });

  return (
    <AccountLayout>
      <PageMeta title="Prompt - Your Dashboard" />

      <section class="py-3 px-3">
        <div class="page-title">
          <h3 class="mb-0">
            {user() ? `Hi ${user()!.username}` : "Hi there"}
          </h3>
          <p class="mt-1 fw-medium">Welcome to Prompt!</p>
          {error() && <p class="text-danger">{error()}</p>}
        </div>

        <Statistics />
        <Projects />
        <Tasks />
      </section>
    </AccountLayout>
  );
};

export default Page;
```