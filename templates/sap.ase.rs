srvbuild.release_directory: {{ sybase_software_dir }}
srvbuild.product: sqlsrv
srvbuild.server_name: {{ sybase_dataserver_name }}
srvbuild.new_config: yes
srvbuild.do_add_server: yes
srvbuild.do_upgrade: no
srvbuild.network_protocol_list: tcp
srvbuild.server_page_size: {{ sybase_logfile_page_size }}
srvbuild.network_hostname_list: {{ ansible_fqdn }}
srvbuild.network_port_list: {{ sybase_port }}
srvbuild.master_device_physical_name: {{ sybase_fsdev_dir }}/${DSQUERYLOWER}master
srvbuild.master_device_size: $masterDEVSIZE
srvbuild.master_database_size: USE_DEFAULT
srvbuild.errorlog: {{ sybase_log_dir }}/errorlog_{{ sybase_dataserver_name }}
srvbuild.sybsystemprocs_device_physical_name: {{ sybasee_fsdev_dir }}/${DSQUERYLOWER}sybprocs
srvbuild.sybsystemprocs_device_size: $sybprocsDEVSIZE
srvbuild.sybsystemprocs_database_size: USE_DEFAULT
srvbuild.sybsystemdb_device_physical_name: USE_DEFAULT
srvbuild.sybsystemdb_device_size: USE_DEFAULT
srvbuild.sybsystemdb_database_size: USE_DEFAULT
srvbuild.default_backup_server: {{ sybase_dataserver_name }}_back
sqlsrv.force_buildmaster: yes
sqlsrv.sa_password: {{ sybase_sa_password }}
