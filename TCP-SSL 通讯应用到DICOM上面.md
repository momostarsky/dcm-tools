####  TCP SSL 协议和DICOM 协议的结合

### 客户端配置采用stunnel 工具配置文件如下:  dicom-storescu 
```ini
[DicomClient]
client = yes
accept = 127.0.0.1:11113
connect = dicom.org.cn:11112
CAfile = /etc/stunnel/client-dicom.crt
verify = 2
```

###  服务器端采用 HAPROXY 配置TCP 端口转发: dicom-storescp
```haproxy.cfg
global
    log         127.0.0.1 local2
    chroot      /var/lib/haproxy
    pidfile     /var/run/haproxy.pid
    user        haproxy
    group       haproxy
    daemon
    maxconn     4000

defaults
    mode                    tcp
    log                     global
    option                  tcplog
    retries                 3
    timeout connect         5s
    timeout client          1m
    timeout server          1m
    maxconn                 3000


# ========== TCP + SSL Frontend ==========
frontend tcp-ssl-in
    bind *:11112 ssl crt /etc/haproxy/dicom.org.cn.pem no-sslv3 no-tls-tickets
    tcp-request inspect-delay 5s
    tcp-request content accept if { req_ssl_hello_type 1 }
    default_backend dicom_servers

# ========== Backend DICOM Servers (plain TCP) ==========
backend dicom_servers
    balance roundrobin
    server dicom_server_1 dicom1.org.cn:11111 check
```
#### 开放11112端口.
```bash
sudo firewall-cmd --state
sudo firewall-cmd --permanent --add-port=11112/tcp
sudo firewall-cmd --reload
```
####  相关证书生成命令如下
```bash
# 生成私钥
openssl genrsa -out dicom.org.cn.key 2048

# 生成证书签名请求 (CSR)
openssl req -new -key dicom.org.cn.key -out dicom.org.cn.csr

# 生成自签名证书
openssl x509 -req -days 365 -in dicom.org.cn.csr -signkey dicom.org.cn.key -out dicom.org.cn.crt


#如果需要，可以通过以下命令将 .key 和 .crt 文件合并成一个 .pem 文件：
cat dicom.org.cn.key dicom.org.cn.crt > dicom.org.cn.pem 
#  .pem 文件中包含了私钥和证书，但私钥不能给客户端，只应提供证书部分。
sudo bash -c "openssl x509 -in /etc/haproxy/dicom.org.cn.pem -out /etc/haproxy/client-dicom.crt"
```

#### 补充建议  openssl.cnf
```text
[ req ]
default_bits        = 2048
distinguished_name  = req_distinguished_name
req_extensions      = req_ext
prompt              = no

[ req_distinguished_name ]
C  = CN
ST = Zhejiang
L  = Hangzhou
O  = Star-sky
OU = Development
CN = DICOM

[ req_ext ]
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = dicom.org.cn
DNS.2 = www.dicom.org.cn
IP.1  = 127.0.0.1
IP.2  = 192.168.2.129
```

### 步骤 2：生成私钥和 CSR（使用上面的配置）

```text
# 生成私钥
openssl genrsa -out dicom.org.cn.key 2048

# 生成带 SAN 的 CSR
openssl req -new -key dicom.org.cn.key -out dicom.org.cn.csr -config openssl.cnf
```

### 步骤 3：生成自签名证书（保留 SAN 扩展）
```text
openssl x509 -req -days 365 -in dicom.org.cn.csr  -signkey dicom.org.cn.key -out dicom.org.cn.crt -ext subjectAltName -extensions req_ext -extfile openssl.cnf
```
###  步骤 4：合并为 PEM 文件（供 HAProxy 使用）
```text
cat dicom.org.cn.key dicom.org.cn.crt > dicom.org.cn.pem
#然后复制到 HAProxy 的配置目录中：
sudo cp dicom.org.cn.pem /etc/haproxy/
sudo chown root:haproxy /etc/haproxy/dicom.org.cn.pem
sudo chmod 600 /etc/haproxy/dicom.org.cn.pem
```

### 步骤 5：导出客户端信任证书（仅公钥部分）
```text
sudo openssl x509 -in /etc/haproxy/dicom.org.cn.pem -out /etc/haproxy/client-dicom.crt
```

#### 一键复制
```text
# 创建 openssl.cnf 文件
cat <<EOL > openssl.cnf
[ req ]
default_bits        = 2048
distinguished_name  = req_distinguished_name
req_extensions      = req_ext
prompt              = no

[ req_distinguished_name ]
C  = CN
ST = Zhejiang
L  = Hangzhou
O  = Star-sky
OU = Development
CN = DICOM

[ req_ext ]
subjectAltName = @alt_names

[ alt_names ]
DNS.1 = dicom.org.cn
DNS.2 = www.dicom.org.cn
IP.1  = 127.0.0.1
IP.2  = 192.168.1.100
EOL

# 生成私钥
openssl genrsa -out dicom.org.cn.key 2048

# 生成 CSR
openssl req -new -key dicom.org.cn.key -out dicom.org.cn.csr -config openssl.cnf

# 生成自签名证书（含 SAN）
openssl x509 -req -days 365 -in dicom.org.cn.csr \
  -signkey dicom.org.cn.key -out dicom.org.cn.crt \
  -ext subjectAltName -extensions req_ext -extfile openssl.cnf

# 合并成 PEM
cat dicom.org.cn.key dicom.org.cn.crt > dicom.org.cn.pem

# 复制到 HAProxy 目录
sudo cp dicom.org.cn.pem /etc/haproxy/
sudo chown root:haproxy /etc/haproxy/dicom.org.cn.pem
sudo chmod 600 /etc/haproxy/dicom.org.cn.pem

# 提取客户端证书
sudo openssl x509 -in /etc/haproxy/dicom.org.cn.pem -out /etc/haproxy/client-dicom.crt

```