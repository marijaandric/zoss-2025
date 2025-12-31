# Domaci 1
CVE-2025-3200

# Domaci 2
CVE-2014-6271 i CVE-2017-16894

# Domaci 3
Eksploatacija CVE-2014-6271, povezivanje Wazuh i TheHive i kreiranje case incident

# Domaci 4
Backup - ansible-playbook -i inventory.ini postgres_backup.yml -k <br>
Restore - ansible-playbook -i inventory.ini postgres_restore.yml -e "local_backup_file=./pg_backups/postgres_backup_2025-12-27_12-21-46.sql" -k <br>
Forensics - ansible-playbook -i inventory.ini shellshock_forensics.yml -k
