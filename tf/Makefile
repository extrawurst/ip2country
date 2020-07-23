KUBE_CONF=.k8/config.yaml
K8_CONF=k8.yaml

tf-plan:
	@terraform plan

tf-apply:
	@terraform apply
	@terraform output k8-config > ${KUBE_CONF}

tf-out:
	@terraform output

tf-destroy:
	@terraform destroy

k8-nodes:
	@kubectl --kubeconfig ${KUBE_CONF} get nodes

k8-apply:
	@kubectl --kubeconfig ${KUBE_CONF} apply -f ${K8_CONF}

k8-delete:
	@kubectl --kubeconfig ${KUBE_CONF} delete -f ${K8_CONF}