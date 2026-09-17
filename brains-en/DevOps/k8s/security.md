# K8s RBAC, Security Policy

### Goal
- RBAC: Role vs Cluster Role
- Hierarchical security design for Pod Security and Network Policy
- Security risks of `hostNetwork: true` setting

## RBAC Role vs ClusterRole

RBAC stands for Role Based Access Control, and it is a k8s permission management system that grants minimum necessary permissions to resources for users or service accounts.

It consists of the following four resources:
1. Role: Namespace-scoped set of permissions
2. ClusterRole: Cluster-wide permissions
3. RoleBinding: Connects a Role to a user service account (namespace-scoped)
4. ClusterRoleBinding: Connects a ClusterRole to a user (can be used globally or without specifying a namespace)

- A Role is **namespace-scoped**, applies to resources within the same namespace, and is bound by a RoleBinding.
- A ClusterRole encompasses **cluster-wide or within-namespace** resources, including all namespaces or cluster resources like nodes/networks, and is bound by a RoleBinding or ClusterRoleBinding.

### Example

```yaml
apiVersion: rback.authorization.k8s.io/v1
kind: Role
metadata:
	namespace: dev
	name: pod-reader
rules:
	- apiGroups: [""]
	  resources: ["pods"]
	  verb: ["get", "list"]
```

```yaml
apiVersion: rback.authorization.k8s.io/v1
kind: ClusterRole
metadata:
	name: node-reader
rules:
	- apiGroups: [""]
	  resources: ["nodes"]
	  verbs: ["get", "list"]
```
> A ClusterRole is essential when allowing cluster-scoped resource access with a ClusterRoleBinding.

<br>

## Pod Security Policy (PSS), Network Policy Security Layer Separation

### PSS(PodSecurity Standard)

Starting from K8s 1.25+, PSP (PodSecurityPolicy) has been deprecated, and a 3-tier security policy is applied via the PodSecurity Admission Controller.
- `privileged`: No restrictions - for operational tools, CNI, etc.
- `baseline`: Allows common workloads - most applications
- `restricted`: Most stringent (e.g., no root) - for security-sensitive applications

> It restricts `runAsNonRoot`, `allowPrivilegeEscalation`, `hostPath` mounts, etc., for Pods.

### NetworkPolicy
- Ingress/Egress control: Configures allowing/blocking inter-pod communication and external communication
- Label selector-based: Defines whether communication is allowed between specific pod groups
- CIDR/port restrictions: Can specify the scope of communication with external entities

### Method for Hierarchical Separation

- PSS (PodSecurity): Controls execution permissions (e.g., prohibits root privileges, host mounts) e.g., `runAsUser`, capabilities restrictions
- RBAC: Controls API resource access, e.g., ConfigMap modification permissions
- NetworkPolicy: Controls network access (inter-pod and external communication control) e.g., only App Pods can access DB Pods

<br>

### `hostNetwork: true` Security Risks

This option allows a pod to share the host's network namespace and use the same network interface as the node's IP.

### Security Risks
- Node-level port conflicts: Risk of conflicts with ports already in use on the host
- Network isolation invalidation: Inter-pod traffic flows through the same network, potentially nullifying network policies
- Host resource access: Potential for accessing host resources like `/proc`, `localhost`, DNS, etc.
- Infection spread: If one pod is compromised, the entire node can be threatened

Activating the `hostNetwork` option is typically used only for infrastructure-level pods like CNI, CoreDNS, etc.
- Examples: node-exporter, fluentbit, istio-cni, etc.

<br>

### Summary

- Role is namespace-scoped, ClusterRole allows cluster-wide access.
- PSS vs NetworkPolicy: PSS restricts pod execution, NetworkPolicy controls network access.
- hostNetwork risks: Port conflicts, network policy bypass, risk of breaking security boundaries.
