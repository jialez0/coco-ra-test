# coco-ra-test

## Build Binaries

```shell
./build.sh
```

## Create an Encrypted and Signed Image

### Prepare Keys

```shell
cd workdir
head -c32 < /dev/random > encryption.key
cosign generate-key-pair
```

### Encrypt Image

Start Sample Keyprovider
```shell
RUST_LOG=debug ./bin/sample_keyprovider
```

Encrypt image: (Use ubuntu:latest for example)
```shell
cd workdir
OCICRYPT_KEYPROVIDER_CONFIG=../ocicrypt.conf skopeo copy --encryption-key provider:attestation-agent:$(realpath encryption.key):kbs://<kbs_ip>:<kbs_port>/default/image_encryption_key/test_1 docker://docker.io/library/ubuntu:latest dir:ubuntu:coco
```
Here, `<kbs_ip>:<kbs_port>` is the address of KBS and `default/image_encryption_key/test_1` is the resource path of the encryption key in KBS, so when register the encryption key to KBS later, the resource path should be consist with here.

### Push Image to Registry

First, Login dockerhub:
```shell
docker login
```
Then you can see auth credential infomation in `/root/.docker/config.json`.

Push image to regiestry:

```shell
skopeo copy dir:ubuntu:coco docker://docker.io/xinjian1326/ubuntu:coco
```

### Sign Image with Cosign

```shell
cd workdir
cosign sign --key cosign.key docker.io/xinjian1326/ubuntu:coco
```

Then edit an image signature policy file:
```json
{
    "default": [{"type": "reject"}], 
    "transports": {
        "docker": {
            "docker.io/xinjian1326/ubuntu": [
                {
                    "type": "sigstoreSigned",
                    "keyPath": "/run/image-security/cosign/cosign.pub"
                }
            ]
        }
    }
}
```


## Pull Image

Start AA:
```shell
export AA_SAMPLE_ATTESTER_TEST=1 # If use real TEE, DO NOT SET THIS!
RUST_LOG=debug ./bin/attestation-agent
```

Start KBS:
```shell
RUST_LOG=debug ./bin/kbs -s <kbs_ip>:<kbs_port>
```

### Register resources to KBS

Register resources:
```shell
cd workdir
kbs_dir=/opt/confidential-containers/kbs/repository/default/
mkdir $kbs_dir/image_encryption_key && mkdir $kbs_dir/cosign_key && mkdir $kbs_dir/image_policy && mkdir $kbs_dir/image_repo_credential
cp encryption.key $kbs_dir/image_encryption_key/test_1
cp cosign.pub $kbs_dir/cosign_key/latest
cp policy.json $kbs_dir/image_policy/latest
cp /root/.docker/config.json $kbs_dir/image_repo_credential/latest
```

### Pull Image with `image-rs`

```shell
cd workdir
export OCICRYPT_KEYPROVIDER_CONFIG=$(pwd)/../ocicrypt.conf
../bin/image-pulling-tool -a -s --aa-kbc-params cc_kbc::http://<kbs_ip>:<kbs_port> --image-url docker.io/xinjian1326/ubuntu:coco
```

# Quick Start

## Pull Existed Encrypted and Signed Image

### Run in Host

```
RUST_LOG=debug ./bin/kbs -s <kbs_ip>:<kbs_port>
```
Register resources:
```shell
kbs_dir=/opt/confidential-containers/kbs/repository/default/
mkdir $kbs_dir/image_encryption_key && mkdir $kbs_dir/cosign_key && mkdir $kbs_dir/image_policy && mkdir $kbs_dir/image_repo_credential
cp data/encryption.key $kbs_dir/image_encryption_key/test_1
cp data/cosign.pub $kbs_dir/cosign_key/latest
cp data/policy.json $kbs_dir/image_policy/latest
cp data/auth.json $kbs_dir/image_repo_credential/latest
```

### Run in TEE

```shell
RUST_LOG=debug ./bin/attestation-agent
```
If use Sample TEE, set Environment:
```shell
export AA_SAMPLE_ATTESTER_TEST=1
```

#### Pull use skopeo
```
cd workdir
OCICRYPT_KEYPROVIDER_CONFIG=../ocicrypt.conf skopeo copy --insecure-policy --decryption-key provider:attestation-agent:cc_kbc::http://<kbs_ip>:<kbs_port> docker://docker.io/xinjian1326/ubuntu:<tag> dir:ubuntu-decrypted-skopeo
```

If KBS is listening on `30.97.46.80:8080`, use `coco` as image tag.
If KBS is listening on `127.0.0.1:8080`, use `local-kbs` as image tag.

#### Pull use `image-rs`
```
cd workdir
export OCICRYPT_KEYPROVIDER_CONFIG=$(pwd)/../ocicrypt.conf
../bin/image-pulling-tool -a -s --aa-kbc-params cc_kbc::http://<kbs_ip>:<kbs_port> --image-url docker.io/xinjian1326/ubuntu:<tag>
```

# Version

The following versions have been tested on TDX:
- Image-rs: `756d4a4`
- AA: `d565529`
- KBS: `48324b2`